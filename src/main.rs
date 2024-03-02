use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[derive(Debug, sqlx::FromRow)]
struct Ex {
    a: i32,
    b: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    println!("Hello, everybody!");

    let conn_opt = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .username("runner")
        .password("not_a_secret")
        .database("foo");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(conn_opt)
        .await
        .expect("failed to connect to database");

    let row = sqlx::query_as::<_, Ex>("select 1 as a, 'abc' as b")
        .fetch_one(&pool)
        .await
        .expect("failed to select");

    eprintln!("{row:#?}");
}
