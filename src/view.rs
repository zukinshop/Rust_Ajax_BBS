use actix_web::{HttpResponse,Responder,get,post,web,Result};
use askama::Template;
use serde_derive::Serialize;
use actix_multipart::form::MultipartForm;
use actix_files::NamedFile;

use crate::models;
use crate::temp;
use crate::forms;

//:.+とすることでサブディレクトリにアクセスできる。
#[get("/static/{filename:.+}")]
async fn show_static(req: web::Path<String>) -> 
//web::Path<String>はワイルドカード({filename})部分にマッチするものを取得する
actix_web::Result<NamedFile>
{
    let file = format!("static/{}", req);
    Ok(NamedFile::open(file)?)
}



#[get("/bbs")]
async fn bbs() -> Result<HttpResponse>  {

    let temp = temp::BbsTemplate{};

    let response_body = temp.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}

#[post("/bbs")]
async fn bbs_post(
    MultipartForm(form): MultipartForm<forms::Report>
    //form:web::Form<forms::Report>
) -> Result<HttpResponse>  {
    
    let _ = models::post_postdata(form);

    let temp = temp::BbsTemplate{};
    
    let response_body = temp.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(response_body))
}

// 新しいAPIエンドポイント
#[get("/api/some_endpoint")]
async fn some_endpoint() -> impl Responder {
    let mut data: Vec<String> = Vec::new();
    //.iter().rev().collect();
    if let Ok(res) = models::get_postdata(){
        data = res;
    };
    
    // サンプルのデータを生成
    let data = SampleData {
        message: data.iter().rev().cloned().collect(),
    };

    // JSON形式でデータを返す
    HttpResponse::Ok().json(data)
}

// サンプルのデータ
#[derive(Debug, Serialize)]
struct SampleData {
    message: Vec<String>,
}
