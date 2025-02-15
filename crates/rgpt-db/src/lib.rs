pub mod schema;

pub mod chat;
pub mod msg;
pub mod session;
pub mod user;

use std::{env, future::Future, process::Command, sync::Arc};

use diesel_async::{AsyncConnection, AsyncPgConnection};
use tokio::sync::Mutex;

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

pub struct Database {
    inner: Mutex<AsyncPgConnection>,
}

impl Database {
    pub async fn establish_conn() -> Database {
        Database {
            inner: Mutex::new(make_conn().await),
        }
    }

    pub async fn establish_arc() -> Arc<Database> {
        Arc::new(Self::establish_conn().await)
    }
}

pub trait RunQueryDsl: diesel_async::RunQueryDsl<AsyncPgConnection> {
    fn execute(
        self,
        conn: Arc<Database>,
    ) -> impl Future<Output = Result<usize, diesel::result::Error>> + Send + 'static
    where
        Self: diesel_async::methods::ExecuteDsl<AsyncPgConnection> + Send + 'static,
    {
        async move {
            let mut conn = conn.inner.lock().await;
            diesel_async::RunQueryDsl::execute(self, &mut conn).await
        }
    }

    fn get_result<U>(
        self,
        conn: Arc<Database>,
    ) -> impl Future<Output = Result<U, diesel::result::Error>> + Send + 'static
    where
        U: Send + 'static,
        Self: diesel_async::methods::LoadQuery<'static, AsyncPgConnection, U> + Send + 'static,
    {
        async move {
            let mut conn = conn.inner.lock().await;
            diesel_async::RunQueryDsl::get_result(self, &mut conn).await
        }
    }

    fn get_results<U>(
        self,
        conn: Arc<Database>,
    ) -> impl Future<Output = Result<Vec<U>, diesel::result::Error>> + Send + 'static
    where
        U: Send + 'static,
        Self: diesel_async::methods::LoadQuery<'static, AsyncPgConnection, U> + Send + 'static,
    {
        async move {
            let mut conn = conn.inner.lock().await;
            diesel_async::RunQueryDsl::get_results(self, &mut conn).await
        }
    }
}

impl<T: diesel_async::RunQueryDsl<AsyncPgConnection>> RunQueryDsl for T {}
