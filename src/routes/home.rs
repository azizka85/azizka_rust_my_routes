use std::collections::HashMap;

use sailfish::TemplateOnce;
use tide::{
  Server, Request, Response, Result, StatusCode, 
  http::mime::{JSON, HTML}
};

use crate::{
  data::{State, pages::HomePage}, 
  helpers::layout::{string_to_array, render_page}
};

pub async fn default<'a>(req: Request<State<'a>>) -> Result<Response> {
  
  let lang = match req.param("lang") {
    Ok(lang) => lang,
    Err(err) => req.state().default_language
  };

  if !req.state().languages.contains_key(lang) {
    return Ok(Response::new(StatusCode::NotFound));
  }

  let translator = req
    .state()
    .languages
    .get(lang)
    .unwrap()
    .translator;

  let mut res = Response::new(StatusCode::Ok);
  
  let query: HashMap<&str, &str> = req.query().unwrap_or_default();

  let ajax = query.contains_key("ajax");
  let init = query.contains_key("init");

  let layouts = query
    .get("layouts")
    .unwrap_or(&"");

  let name = "Aziz";

  if ajax && !init {
    res.set_content_type(JSON)
  } else {
    res.set_content_type(HTML);

    let layouts = 
      if !ajax { 
        vec!["main-layout"] 
      } else { 
        string_to_array(layouts) 
      };
    
    res.set_body(
      render_page(
        ajax, 
        lang, 
        HomePage {
            name
          }
          .render_once()
          .unwrap(), 
        &layouts, 
        &query,
        req.state(),
        translator
      )
    );
  }

  return Ok(res);
}

pub fn register_routes(app: &mut Server<State<'static>>) {
  app.at("/").get(default);
  app.at("/:lang").get(default);
}
