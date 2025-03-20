sqlite 学习

## 1. 导入配置
   > rusqlite = { version = "=0.34.0", features = ["bundled"] }

   >features = ["full"] 含义使用所有的扩展功能

## 2.代码执行

````
use rusqlite::{Connection, Result};
#[derive(Debug)]
struct Person {
id: i32,
name: String,
data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
//Connection::open_in_memory() 是 rusqlite 提供的一个方法，
// 用于创建一个内存中的 SQLite 数据库。
// 与 Connection::open("filename.db") 不同，
// 内存数据库不会将数据持久化到磁盘，而是完全存储在内存中。
// let conn = Connection::open_in_memory()?;
let conn = Connection::open("my_database.db")?;

    // conn.execute(
    //     "CREATE TABLE person (
    //         id    INTEGER PRIMARY KEY,
    //         name  TEXT NOT NULL,
    //         data  BLOB
    //     )",
    //     (), // empty list of parameters.
    // )?;

    let me = Person {
        id: 1,
        name: "Steven1".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;
    

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    
    let id: i32 = 2;
    // 删除满足条件的数据
    let ji = conn.execute("DELETE FROM person WHERE id = ?", [id])?;
    println!("person size = {}", ji);
    
    Ok(())
}

````

## 3.代码说明
 > let conn = Connection::open_in_memory()?;
 ````
 Connection::open_in_memory() 是 rusqlite 提供的一个方法，
 用于创建一个内存中的 SQLite 数据库。
 内存数据库不会将数据持久化到磁盘，而是完全存储在内存中
 ````
 > let conn = Connection::open("my_database.db")?; 
 ````
 Connection::open("filename.db") 是 rusqlite 提供的一个方法，
 用于创建一个持久化到磁盘中 SQLite 数据库。
 filename.db 指定地址
 ````

## 4.查询表是否存在
````
let table_name = "person";
// 查询 sqlite_master 表
let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")?;

// 执行查询
let mut rows = stmt.query([table_name])?;

// 如果查询结果有数据，则表存在
println!("Found person {:?}", rows.next()?.is_some());
````