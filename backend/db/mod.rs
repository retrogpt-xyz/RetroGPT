use std::{env, process::Command};

use diesel::{QueryDsl, SelectableHelper};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use models::NewTmsg;

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

pub async fn get_msgs(conn: &mut AsyncPgConnection) -> Vec<models::Tmsg> {
    use schema::tmsgs::dsl::*;

    tmsgs
        .select(models::Tmsg::as_select())
        .get_results(conn)
        .await
        .unwrap()
}

pub async fn write_msg(
    conn: &mut AsyncPgConnection,
    text: &str,
    prnt: Option<i32>,
) -> models::Tmsg {
    let new_tmsg = NewTmsg {
        body: text.into(),
        prnt_id: prnt,
    };

    use schema::tmsgs;

    diesel::insert_into(tmsgs::table)
        .values(&new_tmsg)
        .returning(models::Tmsg::as_returning())
        .get_result(conn)
        .await
        .unwrap()
}
