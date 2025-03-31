use std::{env, thread, time::Duration};

pub fn startup() {
    // Ensure that the DB service is up and has is
    // ready to accept connections before we continue
    thread::sleep(Duration::from_secs(
        env::var("APP_START_TIMEOUT")
            .unwrap_or("5".to_string())
            .parse()
            .unwrap_or(5),
    ));

    // Run `diesel migration run` to ensure the
    // DB is in sync with the schemas
    //
    // Only really matters in prod. In dev it is
    // assumed that the migrations will be run
    // as you go
    rgpt_db::ensure_migrations();
}
