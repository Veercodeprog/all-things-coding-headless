use axum::routing::get;
use axum::{Extension, Router};
use common::Database;
use std::sync::{Arc, RwLock};
type DatabaseT = Arc<RwLock<Database>>;

async fn try_main() -> anyhow::Result<()> {
    let database = Database::new("0.0.0.0", 3306).expect("Failed to create database");
    let database = Arc::new(RwLock::new(database));
    let app: Router = Router::new()
        // `GET /` goes to `root`
        .route("/hello", get(hello_handler))
        .layer(Extension(database));
    // `POST /users` goes to `create_user`

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    match try_main().await {
        Err(e) => {
            println!("exited program, error: {:?}", e);
        }
        _ => {}
    }
}
async fn hello_handler(Extension(database_lock): Extension<DatabaseT>) -> String {
    match database_lock.read() {
        Ok(database) => {
            let mut output = String::from("Hello World ");

            output.push_str(format!("\n Database IP : {:?}", database.ip).as_str());
            output.push_str(format!("\n Database Port : {:?}", database.port).as_str());

            output
        }
        _ => {
            return String::from("Hello World ");
        }
    }
}
