use crate::db::get_db_connection;
use crate::handlers::{
    create_menu_handler, create_order_handler, create_table_handler,
    delete_item_from_order_handler, get_item_from_order_handler, list_all_order_handler,
    list_menu_handler, list_order_items_from_table_handler, list_table_handler,
};
use rusqlite::Connection;
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if error.if_not_found() {
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error:{:?}", err)),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
    else if let Some(_) = err.find::<warp::filters.body::BodyDeserializeError>(){
        Ok(warp::reply::with_status(
            warp::reply::json(&format!("Error failed to deserialize request body")),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    }
    else{
        Ok(warp::reply_with_status(
            warp_reply_json(&format!("Error {:?}",err)),
            warp::http::StatusCode::INTER_SERVER_ERROR,
        ))
    }
}

pub fn create_order_route() {
    todo!()
}

pub fn create_table_route() {
    todo!()
}

pub fn create_menu_route() {
    todo!()
}

pub fn list_tables_route() {
    todo!()
}

pub fn list_menus_route() {
    todo!()
}

pub fn list_all_orders_route() {
    todo!()
}

pub fn delete_item_from_order_route() {
    todo!()
}

pub fn list_order_items_from_table_route() {
    todo!()
}

pub fn get_item_from_order_route() {
    todo!()
}

pub fn restaurant_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let routes = create_order_route()
        .or(create_table_route())
        .or(create_menu_route())
        .or(list_tables_route())
        .or(list_menus_route())
        .or(list_all_orders_route())
        .or(delete_item_from_order_route())
        .or(list_order_items_from_table_route())
        .or(get_item_from_order_route());

    routes.recover(handle_rejection)
}
