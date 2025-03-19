use sqlx::SqlitePool;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:my_database.db").await?;

    let oldest_user: Vec<Person> = sqlx::query_as("SELECT id, name, data FROM person ORDER BY id")
        .fetch_all(&pool)
        .await?;

    println!("查询结果: {:?}", oldest_user);

    Ok(())
}
