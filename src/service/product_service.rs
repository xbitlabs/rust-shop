use anyhow::anyhow;
use chrono::{Local, Utc};
use sqlx::Arguments;
use sqlx::mysql::MySqlArguments;
use rust_shop_core::db::SqlCommandExecutor;
use rust_shop_core::db::traits::Crud;
use rust_shop_core::id_generator::ID_GENERATOR;
use crate::{dto, vo};
use crate::entity::{Product, ProductCategoryMapping, Sku};
use std::vec::Vec;
use crate::qo::ProductPageQueryRequest;
use crate::utils::{datetime, page_count, page_offset};
use crate::vo::Page;

pub struct ProductService<'a, 'b>{
    sql_command_executor: &'b mut SqlCommandExecutor<'a, 'b>,
}

impl <'a, 'b> ProductService<'a, 'b> {
    pub fn new(sql_command_executor: &'b mut SqlCommandExecutor<'a, 'b>) -> Self {
        ProductService {
            sql_command_executor,
        }
    }
    pub async fn create(&mut self,product:&mut dto::Product)->anyhow::Result<bool>{
        let product_id = ID_GENERATOR.lock().unwrap().real_time_generate();
        let entity = Product{
            id:product_id,
            name: product.name.clone(),
            cover_image: product.cover_image.clone(),
            pics: serde_json::to_string(&product.pics).unwrap(),
            video: product.video.clone(),
            description: product.description.clone(),
            status: product.status.clone(),
            created_time: datetime::now(),
            last_modified_time: None,
            is_deleted: false,
        };

        let mut result =  entity.create(self.sql_command_executor).await?;

        for category_id in &product.category_ids {
            let mut args = MySqlArguments::default();
            args.add(product_id);
            args.add(category_id);
            result = self.sql_command_executor.execute_with("INSERT INTO product_category_mapping(product_id,product_category_id) VALUES(?,?)",args).await? > 0;
        }
        let mut skus = &mut product.skus;
        for mut sku in skus {
            let id = ID_GENERATOR.lock().unwrap().real_time_generate();
            sku.id = id;
            sku.product_id = product_id;
            result = sku.create(self.sql_command_executor).await?;
        }
        Ok(result)
    }
    pub async fn update(&mut self,product:&mut dto::Product)->anyhow::Result<bool>{
        let mut entity = Product::select_by_id(product.id,self.sql_command_executor).await?;
        if entity.is_some() {
            let mut entity = entity.unwrap();
            entity.video = product.video.clone();
            entity.pics = serde_json::to_string(&product.pics).unwrap();
            entity.description = product.description.clone();
            entity.cover_image = product.cover_image.clone();
            entity.name = product.name.clone();
            entity.last_modified_time = Some(Utc::now());
            let mut result = entity.update(self.sql_command_executor).await?;

            let mut args = MySqlArguments::default();
            args.add(product.id);
            let db_exists_skus:Vec<Sku> = self.sql_command_executor.find_all_with("SELECT * FROM sku WHERE product_id=?",args).await?;
            let mut new_skus = vec![];
            let mut modified_skus = vec![];
            for mut sku in &mut product.skus {
                if sku.id <= 0 {
                    sku.id = ID_GENERATOR.lock().unwrap().real_time_generate();
                    new_skus.push(sku);
                }else {
                    modified_skus.push(sku);
                }
            }
            if !new_skus.is_empty() {
                for new_sku in new_skus {
                    result = new_sku.create(self.sql_command_executor).await?;
                }
            }
            if !modified_skus.is_empty() {
                for modified_sku in modified_skus {
                    result = modified_sku.update(self.sql_command_executor).await?;
                }
            }
            let deleted_skus:Vec<&Sku> = db_exists_skus.iter().filter(|item|{
                for sku in &product.skus {
                    if sku.id == item.id {
                        return false;
                    }
                }
                return true;
            }).collect();
            if !deleted_skus.is_empty() {
                for deleted_sku in deleted_skus {
                    //result = Sku::delete_by_id(deleted_sku.id,self.sql_command_executor).await?;
                    let mut args = MySqlArguments::default();
                    args.add(deleted_sku.id);
                    result = self.sql_command_executor.execute_with("UPDATE sku set is_deleted = 1 WHERE id=?",args).await? > 0;
                }
            }
            let mut args = MySqlArguments::default();
            args.add(product.id);
            result = self.sql_command_executor.execute_with("DELETE FROM product_category_mapping WHERE product_id=?",args).await? > 0;
            for category_id in &product.category_ids {
                let mut args = MySqlArguments::default();
                args.add(product.id);
                args.add(category_id);
                result = self.sql_command_executor.execute_with("INSERT INTO product_category_mapping(product_id,product_category_id) VALUES(?,?)",args).await? > 0;
            }
            Ok(result)
        }else {
            Err(anyhow!(format!("not found product by id {},update failed!",product.id)))
        }
    }
    pub async fn delete_by_id(&mut self,id:i64)->anyhow::Result<bool> {
        let result = Product::delete_by_id(id, self.sql_command_executor).await?;
        Ok(result)
    }
    pub async fn mark_deleted(&mut self,id:i64)->anyhow::Result<bool>{
        let product = Product::select_by_id(id,self.sql_command_executor).await?;
        if product.is_some() {
            let mut product = product.unwrap();
            product.is_deleted = true;
            let result = product.update(self.sql_command_executor).await?;
            Ok(result)
        }else {
            Ok(false)
        }
    }
    pub async fn get_by_id(&mut self,id:i64)->anyhow::Result<vo::Product>{
        let product = Product::select_by_id(id,self.sql_command_executor).await?;
        if product.is_some() {
            let product = product.unwrap();
            let mut args = MySqlArguments::default();
            args.add(product.id);
            let product_categories:Vec<ProductCategoryMapping> = self.sql_command_executor.find_all_with("SELECT * FROM product_category_mapping WHERE product_id=?",args).await?;
            let product_category_ids = product_categories.iter().map(|item|{
                item.product_category_id
            }).collect();

            let mut args = MySqlArguments::default();
            args.add(product.id);
            let skus:Vec<Sku> = self.sql_command_executor.find_all_with("SELECT * FROM sku WHERE product_id = ? AND is_deleted = 0",args).await?;

            let vo = vo::Product{
                id:product.id,
                name: product.name.clone(),
                cover_image: product.cover_image.clone(),
                pics: serde_json::from_str(product.pics.as_str()).unwrap(),
                video: product.video,
                description: product.description.clone(),
                status: product.status,
                created_time: product.created_time,
                last_modified_time: product.last_modified_time,
                is_deleted: product.is_deleted,
                product_category_ids,
                skus,
            };
            Ok(vo)
        }else {
            Err(anyhow!(format!("not found ProductCategory by id：{}",id)))
        }
    }
    pub async fn page(&mut self,page_query:&ProductPageQueryRequest)-> anyhow::Result<Page<vo::Product>>{
        let mut sql = String::from("SELECT * FROM product p");
        let mut count_sql = String::from("SELECT COUNT(*) FROM product p");
        let mut args = MySqlArguments::default();
        let mut count_args = MySqlArguments::default();
        let mut sql_where = vec![];
        if page_query.category_id.is_some() {
            sql = sql + " JOIN product_category_mapping m ON p.id = m.product_id ";
            count_sql = count_sql + " JOIN product_category_mapping m ON p.id = m.product_id ";
            sql_where.push(String::from(" m.product_category_id = ? "));
            args.add(page_query.category_id.unwrap());
            count_args.add(page_query.category_id.unwrap());
        }
        if page_query.keyword.is_some() {
            sql_where.push(String::from(" p.name like ? "));
            args.add(String::from("%") + &*page_query.keyword.as_ref().unwrap().clone() + "%");
            count_args.add(String::from("%") + &*page_query.keyword.as_ref().unwrap().clone() + "%");
        }
        if page_query.status.is_some() {
            sql_where.push(String::from(" p.status = ? "));
            args.add(page_query.status.as_ref().unwrap().clone());
            count_args.add(page_query.status.as_ref().unwrap().clone());
        }
        if !sql_where.is_empty() {
            let last = sql_where.last().unwrap().clone();
            sql = sql + " WHERE ";
            count_sql = count_sql + " WHERE ";
            for s in sql_where {
                if s != last {
                    sql = sql + " AND " + s.as_str();
                    count_sql = count_sql + " AND " + s.as_str();
                }else {
                    sql = sql + " " + s.as_str();
                    count_sql = count_sql + " " + s.as_str();
                }
            }
        }

        let record_count:i64 = self.sql_command_executor.scalar_one_with(count_sql.as_str(),count_args).await?;

        if page_query.sort.is_none() {
            sql = sql + " ORDER BY id DESC ";
        }
        sql = sql + " LIMIT " + &*page_offset(page_query.page_size, page_query.page_index).to_string() + ",10";
        let products:Vec<Product> = self.sql_command_executor.find_all_with(sql.as_str(),args).await?;

        let mut vos:Vec<vo::Product> = vec![];
        for product in products {
            let mut vo = vo::Product{
                id: product.id,
                name: product.name,
                cover_image: product.cover_image,
                pics: serde_json::from_str(&*product.pics).unwrap(),
                video: product.video,
                description: product.description,
                status: product.status,
                created_time: product.created_time,
                last_modified_time: product.last_modified_time,
                is_deleted: product.is_deleted,
                product_category_ids: vec![],
                skus: vec![],
            };
            let mut args = MySqlArguments::default();
            args.add(product.id);
            let skus:Vec<Sku> = self.sql_command_executor.find_all_with("SELECT * FROM sku where product_id=?",args).await?;
            vo.skus = skus;

            let mut args = MySqlArguments::default();
            args.add(product.id);
            let category_mapping:Vec<ProductCategoryMapping> = self.sql_command_executor.find_all_with("SELECT * FROM product_category_mapping WHERE product_id=?",args).await?;
            let category_ids : Vec<i64> = category_mapping.iter().map(|item| item.product_category_id).collect();
            vo.product_category_ids = category_ids;
            vos.push(vo);
        }
        let page = vo::Page{
            page_size: page_query.page_size,
            page_index: page_query.page_index,
            page_count: page_count(record_count,page_query.page_size),
            record_count,
            items: vos,
        };
        Ok(page)
    }
}