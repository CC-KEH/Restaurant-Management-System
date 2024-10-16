mod db;
mod handlers;
mod models;
mod routes;
use warp::Filter;

#[tokio::main]
async fn main() {
    db::initialize_db();
    let routes = routes::restaurant_routes();

    println!("Running the server");
    warp::serve(routes.with(warp::trace::request()))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
