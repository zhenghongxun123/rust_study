sqlite 学习

## 1. 导入配置


````
tokio = { version = "1.44.1", features = ["full"] }
sqlx = { version = "=0.8.1", features = ["sqlite", "runtime-tokio", "tls-native-tls"] }
rusqlite = { version = "=0.32.1",features = ["bundled"] }

````
> 请注意 sqlx 和 rusqlite 都会引入库 libsqlite3-sys .可能会导致编译失败。如果出现编译问题，降低rusqlite的版本


## 2.代码执行

````
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

````

## 3.代码说明
 > let pool = SqlitePool::connect("sqlite://D:/05XIANGMU/rust/rust_study/my_database.db").await?;
 ````
sqlite://：这是 sqlx 中 SQLite 连接字符串的前缀。
绝对路径：在 sqlite:// 后面直接跟上数据库文件的绝对路径。
在 Unix/Linux/macOS 系统中，路径格式为 /home/user/databases/my_database.db。
在 Windows 系统中，路径格式为 C:/Users/User/databases/my_database.db（注意使用正斜杠 / 而不是反斜杠 \）。
 ````
 > let pool = SqlitePool::connect("sqlite:my_database.db").await?;
 ````
相对路径：在 sqlite: 后面直接跟上数据库文件的相对路径。
 ````

## 4.增删改查
````
上面添加表、删除表、添加数据、删除数据、查询数据
````