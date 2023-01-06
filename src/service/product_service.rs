use anyhow::anyhow;
use chrono::Utc;
use sqlx::Arguments;
use sqlx::mysql::MySqlArguments;
use rust_shop_core::db::SqlCommandExecutor;
use rust_shop_core::db::traits::Crud;
use rust_shop_core::id_generator::ID_GENERATOR;
use crate::{dto, vo};
use crate::entity::{Product, ProductCategoryMapping, Sku};
use std::vec::Vec;

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
            created_time: Utc::now(),
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
    pub async fn update(&mut self,product:&dto::Product)->anyhow::Result<bool>{
        let mut entity = Product::select_by_id(product.id,self.sql_command_executor).await?;
        if entity.is_some() {
            let mut entity = entity.unwrap();
            entity.video = product.video.clone();
            entity.pics = serde_json::to_string(&product.pics).unwrap();
            entity.description = product.description.clone();
            entity.cover_image = product.cover_image.clone();
            entity.name = product.name.clone();
            entity.last_modified_time = Some(Utc::now());
            let result = entity.update(self.sql_command_executor).await?;

            let mut args = MySqlArguments::default();
            args.add(product.id);
            let db_exists_skus:Vec<Sku> = self.sql_command_executor.find_all_with("SELECT * FROM sku WHERE product_id=?",args).await?;
            let mut new_skus = vec![];
            let mut modified_skus = vec![];
            for sku in &product.skus {
                if sku.id >= 0 {
                    new_skus.push(sku);
                }else {
                    modified_skus.push(sku);
                }
            }
            let deleted_sku = db_exists_skus.iter().filter(|item|{
                let items = product.skus.iter().filter(|i|{i.id == item.id}).
            }).collect();

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
}