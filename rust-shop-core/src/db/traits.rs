use std::pin::Pin;
use anyhow::anyhow;

use futures::stream::Stream;
use futures::stream::TryCollect;
use futures::Future;
use futures::{future, TryFutureExt, TryStreamExt};
use sqlx::database::HasArguments;
use sqlx::query::QueryAs;
use sqlx::{Arguments, Database, Encode, Execute, Executor, FromRow, IntoArguments, MySql, Type};
use sqlx::mysql::{MySqlArguments, MySqlRow};
use sqlx_crud::traits;
use crate::db::SqlCommandExecutor;

/// Type alias for methods returning a single element. The future resolves to and
/// `Result<T, sqlx::Error>`.
pub type CrudFut<'e, T> = Pin<Box<dyn Future<Output = Result<T, sqlx::Error>> + 'e + Send>>;

/// Type alias for a [`Stream`] returning items of type `Result<T, sqlxError>`.
pub type CrudStream<'e, T> =
    Pin<Box<dyn Stream<Item = Result<T, sqlx::Error>> + std::marker::Send + 'e>>;

/// Type alias for a [`TryCollect`] future that resolves to `Result<Vec<T>, sqlx::Error>`.
pub type TryCollectFut<'e, T> = TryCollect<CrudStream<'e, T>, Vec<T>>;

/// Database schema information about a struct implementing sqlx [FromRow].
/// [Schema] defines methods for accessing the derived database schema
/// and query information.
///
/// This trait is implemented by the [SqlxCrud] derive macro.
///
/// # Example
///
/// ```rust
/// use sqlx::FromRow;
/// use sqlx_crud::SqlxCrud;
///
/// #[derive(FromRow, SqlxCrud)]
/// pub struct User {
///     user_id: i32,
///     name: String,
/// }
/// ```
///
/// [FromRow]: https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html
pub trait Schema {
    /// Type of the table primary key column.
    type Id: Send;

    /// Database name of the table. Used by the query generation code and
    /// available for introspection. This is generated by taking the plural
    /// _snake_case_ of the struct's name. See: [Inflector to_table_case].
    ///
    /// ```rust
    /// use sqlx::FromRow;
    /// use sqlx_crud::{Schema, SqlxCrud};
    ///
    /// #[derive(FromRow, SqlxCrud)]
    /// struct GoogleIdToken {
    ///     id: i32,
    ///     audience: String,
    /// }
    ///
    /// assert_eq!("google_id_tokens", GoogleIdToken::table_name());
    /// ```
    ///
    /// [Inflector to_table_case]: https://docs.rs/Inflector/latest/inflector/cases/tablecase/fn.to_table_case.html
    fn table_name() -> &'static str;

    /// Returns the id of the current instance.
    fn id(&self) -> Self::Id;

    /// Returns the column name of the primary key.
    fn id_column() -> &'static str;

    /// Returns an array of column names.
    fn columns() -> &'static [&'static str];

    /// Returns the SQL string for a SELECT query against the table.
    ///
    /// # Example
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx_crud::Schema;
    ///
    /// assert_eq!(r#"SELECT "users"."user_id", "users"."name" FROM "users""#, User::select_sql());
    /// # }}
    /// ```
    fn select_sql() -> &'static str;

    /// Returns the SQL string for a SELECT query against the table with a
    /// WHERE clause for the primary key.
    ///
    /// # Example
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx_crud::Schema;
    ///
    /// assert_eq!(
    ///     r#"SELECT "users"."user_id", "users"."name" FROM "users" WHERE "users"."user_id" = ? LIMIT 1"#,
    ///     User::select_by_id_sql()
    /// );
    /// # }}
    /// ```
    fn select_by_id_sql() -> &'static str;

    /// Returns the SQL for inserting a new record in to the database. The
    /// `#[external_id]` attribute may be used to specify IDs are assigned
    /// outside of the database.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx::FromRow;
    /// use sqlx_crud::{Schema, SqlxCrud};
    ///
    /// #[derive(Debug, FromRow, SqlxCrud)]
    /// #[external_id]
    /// pub struct UserExternalId {
    ///     pub user_id: i32,
    ///     pub name: String,
    /// }
    ///
    /// assert_eq!(r#"INSERT INTO "users" ("name") VALUES (?) RETURNING "users"."user_id", "users"."name""#, User::insert_sql());
    /// assert_eq!(r#"INSERT INTO "user_external_ids" ("user_id", "name") VALUES (?, ?) RETURNING "user_external_ids"."user_id", "user_external_ids"."name""#, UserExternalId::insert_sql());
    /// # }}
    /// ```
    fn insert_sql() -> &'static str;

    /// Returns the SQL for updating an existing record in the database.
    ///
    /// # Example
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx_crud::Schema;
    ///
    /// assert_eq!(r#"UPDATE "users" SET "name" = ? WHERE "users"."user_id" = ? RETURNING "users"."user_id", "users"."name""#, User::update_by_id_sql());
    /// # }}
    /// ```
    fn update_by_id_sql() -> &'static str;

    /// Returns the SQL for deleting an existing record by ID from the database.
    ///
    /// # Example
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx_crud::Schema;
    ///
    /// assert_eq!(r#"DELETE FROM "users" WHERE "users"."user_id" = ?"#, User::delete_by_id_sql());
    /// # }}
    /// ```
    fn delete_by_id_sql() -> &'static str;
}

/// Common Create, Read, Update, and Delete behaviors. This trait requires that
/// [Schema] and [FromRow] are implemented for Self.
///
/// This trait is implemented by the [SqlxCrud] derive macro. Implementors
/// define how to assign query insert and update bindings.
///
/// [FromRow]: https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html
/// [Schema]: trait.Schema.html
/// [SqlxCrud]: ../derive.SqlxCrud.html
pub trait Crud<'e>
where
    Self: 'e + Sized + Send + Unpin + Schema,
{
    /// Given a query returns a new query with parameters suitable for an
    /// INSERT bound to it. The [SqlxCrud] implementation will bind the
    /// primary key, and each additional field to the query.
    ///
    /// # Example
    ///
    /// Typically you would just use `<Self as Crud>::insert()` but if you
    /// wanted to modify the query you could use something like:
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx_crud::{Crud, Schema};
    ///
    /// let user = User { user_id: 1, name: "Test".to_string() };
    /// let query = sqlx::query_as::<_, User>(User::insert_sql());
    /// let query = user.insert_binds(query);
    /// let user = query.fetch_one(&pool).await?;
    /// assert_eq!("Test", user.name);
    ///
    /// # }}
    /// ```
    ///
    /// This would bind `user_id`, `name` to the query in that order.
    ///
    /// [SqlxCrud]: ../derive.SqlxCrud.html
    fn insert_binds(
        &'e self,
        query: QueryAs<'e, MySql, Self, MySqlArguments>,
    ) -> QueryAs<'e, MySql, Self, MySqlArguments>;

    /// Given a query returns a new query with parameters suitable for an
    /// UPDATE bound to it. The [SqlxCrud] implementation will bind every
    /// column except for the primary key followed by the primary key to
    /// the query.
    ///
    /// # Example
    ///
    /// Typically you would just use `<Self as Crud>::update()` but if you
    /// wanted to modify the query you could use something like:
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx_crud::{Crud, Schema};
    ///
    /// let user = User { user_id: 1, name: "other".to_string() };
    /// let query = sqlx::query_as::<_, User>(User::update_by_id_sql());
    /// let query = user.update_binds(query);
    /// let user = query.fetch_one(&pool).await?;
    /// assert_eq!("other", user.name);
    ///
    /// # }}
    /// ```
    ///
    /// This would bind `name`, `user_id` to the query in that order.
    ///
    /// [SqlxCrud]: ../derive.SqlxCrud.html
    fn update_binds(
        &'e self,
        query: QueryAs<'e, MySql, Self, MySqlArguments>,
    ) -> QueryAs<'e, MySql, Self, MySqlArguments>;

    /// Returns a future that resolves to an insert or `sqlx::Error` of the
    /// current instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx_crud::{Crud, Schema};
    ///
    /// let user = User { user_id: 1, name: "test".to_string() };
    /// let user = user.create(&pool).await?;
    /// assert_eq!("test", user.name);
    /// # }}
    /// ```
    fn create(&'e self, sql_exe: &'e mut SqlCommandExecutor<'_, '_>) -> CrudFut<'e, bool> where for<'r> Self: FromRow<'r, MySqlRow>, Self: Sync{
        match sql_exe {
            SqlCommandExecutor::WithTransaction(tran) => {
                Box::pin(async move {
                    let query = sqlx::query_as::<MySql, Self>(<Self as Schema>::insert_sql());
                    let query = self.insert_binds(query);
                    //let r = query.statement(tran.transaction()).await?;
                    let result = tran.transaction().execute(query).await?;
                    Ok(result.rows_affected() > 0)
                })
            }
            SqlCommandExecutor::WithoutTransaction(exe) => {
                Box::pin(async move {
                    let query = sqlx::query_as::<MySql, Self>(<Self as Schema>::insert_sql());
                    let query = self.insert_binds(query);
                    //let r = query.fetch_one(*exe).await?;
                    let result = exe.execute(query).await?;
                    Ok(result.rows_affected() > 0)
                })
            }
        }

    }

    /// Queries all records from the table and returns a future that returns
    /// to a [try_collect] stream, which resolves to a `Vec<Self>` or a
    /// `sqlx::Error` on error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx_crud::Crud;
    ///
    /// let all_users: Vec<User> = User::all(&pool).await?;
    /// # }}
    /// ```
    ///
    /// [try_collect]: https://docs.rs/futures/latest/futures/stream/trait.TryStreamExt.html#method.try_collect
    fn all(sql_exe: &'e mut SqlCommandExecutor<'_, '_>) -> TryCollectFut<'e, Self> where for<'r> Self: FromRow<'r, MySqlRow>{
        match sql_exe {
            SqlCommandExecutor::WithTransaction(tran) => {
                let stream =
                    sqlx::query_as::<MySql, Self>(<Self as Schema>::select_sql()).fetch(tran.transaction());
                stream.try_collect()
            }
            SqlCommandExecutor::WithoutTransaction(pool) => {
                let stream =
                    sqlx::query_as::<MySql, Self>(<Self as Schema>::select_sql()).fetch(*pool);
                stream.try_collect()
            }
        }

    }

    #[doc(hidden)]
    fn paged(sql_exe: &'e mut SqlCommandExecutor<'_, '_>) -> TryCollectFut<'e, Self> {
        unimplemented!()
    }

    /// Looks up a row by ID and returns a future that resolves an
    /// `Option<Self>`. Returns `None` if and a record with the corresponding ID
    /// cannot be found and `Some` if it exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx_crud::Crud;
    ///
    /// let user: Option<User> = User::by_id(&pool, 1).await?;
    /// assert!(user.is_some());
    /// # }}
    /// ```
    fn select_by_id(sql_exe: &'e mut SqlCommandExecutor<'_, '_>, id: <Self as Schema>::Id) -> CrudFut<'e, Option<Self>> where for<'r> Self: FromRow<'r, MySqlRow>, <Self as Schema>::Id: Encode<'e, MySql>, Self: Schema, <Self as Schema>::Id: Type<MySql>{
        match sql_exe {
            SqlCommandExecutor::WithTransaction(tran) => {
                Box::pin(
                    sqlx::query_as::<MySql, Self>(<Self as Schema>::select_by_id_sql())
                        .bind(id)
                        .fetch_optional(tran.transaction()),
                )
            }
            SqlCommandExecutor::WithoutTransaction(pool) => {
                Box::pin(
                    sqlx::query_as::<MySql, Self>(<Self as Schema>::select_by_id_sql())
                        .bind(id)
                        .fetch_optional(*pool),
                )
            }
        }

    }

    /// Updates the database with the current instance state and returns a
    /// future that resolves an `()` on success and `sqlx::Error` on error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx_crud::Crud;
    ///
    /// if let Some(mut user) = User::by_id(&pool, 1).await? {
    ///     user.name = "Harry".to_string();
    ///     let user = user.update(&pool).await?;
    ///     assert_eq!("Harry", user.name);
    /// }
    /// # }}
    /// ```
    fn update(&'e self, sql_exe: &'e mut SqlCommandExecutor<'_, '_>) -> CrudFut<'e, bool> where for<'r> Self: FromRow<'r, MySqlRow>, Self: Sync{
        match sql_exe {
            SqlCommandExecutor::WithTransaction(tran) => {
                Box::pin(async move {
                    let query = sqlx::query_as::<MySql, Self>(<Self as Schema>::update_by_id_sql());
                    let query = self.update_binds(query);
                    //let r = query.fetch_one(tran.transaction()).await?;
                    let result = tran.transaction().execute(query).await?;
                    Ok(result.rows_affected() > 0)
                })
            }
            SqlCommandExecutor::WithoutTransaction(pool) => {
                Box::pin(async move {
                    let query = sqlx::query_as::<MySql, Self>(<Self as Schema>::update_by_id_sql());
                    let query = self.update_binds(query);
                    //let r = query.fetch_one(*pool).await?;

                    let result = pool.execute(query).await?;
                    Ok(result.rows_affected() > 0)
                })
            }
        }

    }

    /// Deletes a record from the database by ID and returns a future that
    /// resolves to `()` on success or `sqlx::Error` on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// # sqlx_crud::doctest_setup! { |pool| {
    /// use sqlx_crud::Crud;
    ///
    /// if let Some(user) = User::by_id(&pool, 1).await? {
    ///     user.delete(&pool).await?;
    /// }
    /// assert!(User::by_id(&pool, 1).await?.is_none());
    /// # }}
    /// ```
    fn delete_by_id(id:Self::Id,  sql_exe: &'e mut SqlCommandExecutor<'_, '_>) -> CrudFut<'e, bool> where <Self as Schema>::Id: Encode<'e, MySql>, <Self as Schema>::Id: Type<MySql>{
        Box::pin(async move {
            let sql = <Self as Schema>::delete_by_id_sql();
            let mut args = MySqlArguments::default();
            args.add(id);
            return match sql_exe {
                SqlCommandExecutor::WithTransaction(ref mut tran_manager) => {
                    let result = sqlx::query_with(sql, args)
                        .execute(tran_manager.transaction())
                        .await?;
                    Ok(result.rows_affected() > 0)
                }
                SqlCommandExecutor::WithoutTransaction(pool) => {
                    let result = sqlx::query_with(sql, args).execute(*pool).await?;
                    Ok(result.rows_affected() > 0)
                }
            };
        })
    }
}
