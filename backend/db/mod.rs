use std::{env, process::Command};

use diesel_async::{AsyncConnection, AsyncPgConnection};

pub mod models;
pub mod schema;

pub fn ensure_migrations() {
    let container_db_url = env::var("CONTAINER_DATABASE_URL").unwrap();
    let outp = Command::new("./diesel")
        .args(["migration", "run", "--database-url", &container_db_url])
        .output()
        .unwrap();

    println!("{}", String::from_utf8_lossy(&outp.stdout));
}

pub async fn make_conn() -> AsyncPgConnection {
    let db_url = std::env::var("CONTAINER_DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{db_url}");
    AsyncPgConnection::establish(&db_url).await.unwrap()
}
