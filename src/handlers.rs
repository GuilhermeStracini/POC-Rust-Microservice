use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::models::Item;

pub async fn create_item(item: web::Json<Item>) -> impl Responder {
    HttpResponse::Created().json(item.into_inner())
}

pub async fn get_items() -> impl Responder {
    let items = vec![
        Item::new(1, "Item 1", "Description 1"),
        Item::new(2, "Item 2", "Description 2"),
    ];
    HttpResponse::Ok().json(items)
}

pub async fn get_item(item_id: web::Path<u32>) -> impl Responder {
    let item = Item::new(item_id.into_inner(), "Item", "Description");
    HttpResponse::Ok().json(item)
}

pub async fn update_item(item_id: web::Path<u32>, item: web::Json<Item>) -> impl Responder {
    let updated_item = Item::new(item_id.into_inner(), &item.name, &item.description);
    HttpResponse::Ok().json(updated_item)
}

pub async fn delete_item(item_id: web::Path<u32>) -> impl Responder {
    HttpResponse::NoContent().finish()
}

// This module contains handlers for CRUD operations on the Item resource.
// These handlers will be used to manage the lifecycle of Item instances in the microservice.
