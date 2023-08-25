use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use dotenv::dotenv

#[macro_use] extern crate rocket;

pub struct App_state{
    db: Pool<Postgres>,
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok()
    rocket::build().mount("/", routes![index])

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URl must be set")
    let pool= match PgPoolOptions::new()
        .max_connection(10)
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
    

    
}