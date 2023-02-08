use crate::custom_response::HtmlTemplate;
use askama::Template;
use crate::{
     Result,
};
#[derive(Template)]
#[template(path = "index.html")]
pub struct HomePageTemplate<'a> {
    data: &'a str,
}

pub async fn home_page<'a>(
 ) -> Result<HtmlTemplate<HomePageTemplate<'a>>> {
    let a = HomePageTemplate { data: "data" };
    Ok(HtmlTemplate(a))
}

#[derive(Template)]
#[template(path = "workspace.html")]
pub struct WorkSpaceTemplate<'a> {
    data: &'a str,
}

pub async fn workspace<'a>(
 ) -> Result<HtmlTemplate<WorkSpaceTemplate<'a>>> {
    let a = WorkSpaceTemplate { data: "data" };
    Ok(HtmlTemplate(a))
}