use askama::Template;

#[derive(Template)]
#[template(path="bbs.html")]
pub struct BbsTemplate {}

