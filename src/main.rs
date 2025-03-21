use sqlx::SqlitePool;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite://D:/05XIANGMU/rust/rust_study/my_database.db").await?;
    // let pool = SqlitePool::connect("sqlite:my_database.db").await?;

    let table_name: (Option<String>,) =
        sqlx::query_as("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
            .bind("person1")
            .fetch_one(&pool)
            .await?;

    match table_name {
        (Some(name),) => {
            println!("表名: {}", name);

            sqlx::query("DROP TABLE IF EXISTS person1")
                .execute(&pool)
                .await?;
        }
        (None,) => {
            println!("表 'person1' 不存在");
        }
    }

    //创建表
    let create_table_sql = r#" CREATE TABLE person1 (
        id    INTEGER PRIMARY KEY,
        name  TEXT NOT NULL,
        data  BLOB
       )"#;
    sqlx::query(create_table_sql).execute(&pool).await?;

    let result = sqlx::query("INSERT INTO person1 (name) VALUES (?1)")
        .bind("Steven2")
        .execute(&pool)
        .await?;
    println!("插入成功，受影响的行数: {}", result.rows_affected());

    let oldest_user: Vec<Person> = sqlx::query_as("SELECT id, name, data FROM person1 ORDER BY id")
        .fetch_all(&pool)
        .await?;

    println!("查询结果: {:?}", oldest_user);

    let result = sqlx::query("delete from  person1 where id = ?1")
        .bind(1)
        .execute(&pool)
        .await?;
    println!("删除成功，受影响的行数: {}", result.rows_affected());

    Ok(())
}
