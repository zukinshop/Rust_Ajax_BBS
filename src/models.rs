use rusqlite::{Connection, Result};

use crate::forms;

pub fn create_table()->Result<()>{
    let mut conn = Connection::open("db.sqlite3")?;

    // トランザクションを開始
    let transaction = conn.transaction()?;

    // テーブル作成を行うSQL文

    //ユーザー情報のテーブル作成
    let sql = "
    CREATE TABLE IF NOT EXISTS postdata (
        id INTEGER PRIMARY KEY,

        data TEXT
    ) ;
    ";

    transaction.execute(sql,[])?;

    // トランザクションをコミット
    transaction.commit()


    
}

pub fn get_postdata() ->Result<Vec<String>>{
    let mut vec:Vec<String> = Vec::new();

    struct DataList {
        data: String,
    }

    let conn = Connection::open("db.sqlite3")?;

    // SQLクエリを実行してユーザーを検索
    let mut stmt = conn.prepare("SELECT data FROM postdata")?;

    let post_iter = stmt.query_map([], |row| {
        Ok(DataList {
            //id: row.get(0)?,
            data: row.get("data")?,
        })
    })?;

    for i  in post_iter{
            let post: DataList = i?;

            let result_post: String = post.data;

            vec.push(result_post);
    };
    

    return Ok(vec)

}

pub fn post_postdata(data:forms::Report) ->Result<()>{
    let mut conn = Connection::open("db.sqlite3")?;

    // トランザクションを開始
    let transaction = conn.transaction()?;

    //ユーザー情報のテーブル作成
    let sql = "
    INSERT INTO postdata (data) VALUES (?1); 
    ";
    
    transaction.execute(sql,[&data.report.into_inner()])?;

    // トランザクションをコミット
    transaction.commit()?;

    Ok(())
}