use actix_web::{
    App,
    HttpServer,
};

pub mod view;
pub mod models;
pub mod temp;
pub mod forms;

//サーバーの名前
const SERVER_ADDR:&str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("http://{SERVER_ADDR}");

    let _ = models::create_table();
    
    //サーバー設定
    HttpServer::new(move||{
        App::new()
        
        //静的ファイルの提供
        .service(view::show_static)
        
        .service(view::bbs)
        .service(view::some_endpoint)
        .service(view::bbs_post)

    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}

