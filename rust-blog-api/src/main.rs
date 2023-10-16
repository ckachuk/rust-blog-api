
#[macro_use] extern crate rocket;

pub mod models;
pub mod controllers;
pub mod validators; 
pub mod services;
pub mod utils;

use controllers::post_controller;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use crate::controllers::category_controller;
use crate::controllers::user_controller;
use crate::utils::catchers; 

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}




#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    dotenv().ok();
    
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URl must be set");
    let pool: sqlx::Pool<sqlx::Postgres> = match PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    {
            Ok(pool) => {
                println!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
    };
   
    
    rocket::build()
    .mount("/", routes![index, 
    category_controller::create_category_controller, 
    category_controller::get_categories_controller, 
    category_controller::get_category_controller, 
    category_controller::delete_categoy_controller, 
    category_controller::update_category_controller,
    user_controller::sign_up_controller,
    user_controller::login_controller,
    user_controller::change_password,
    post_controller::create_post_controller,
    post_controller::get_posts_controller,
    post_controller::get_user_posts_controller,
    post_controller::update_post_controller
    ])
    .register("/",rocket::catchers![catchers::bad_request, catchers::not_found, catchers::unauthorized])
    .manage(pool)
    
}