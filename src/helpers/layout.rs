use std::collections::HashMap;

use azizka_rust_i18n::Translator;
use sailfish::TemplateOnce;

use crate::data::{layouts::{MainLayout, DefaultLayout}, State};

pub fn string_to_array(param: &str) -> Vec<&str> {
  param
    .split(",")
    .map(|item| item.trim())
    .collect()
}

pub fn render_page<'a>(
  ajax: bool,
  lang: &'a str,
  content: String,
  layouts: &'a Vec<&'a str>,
  query: &HashMap<&'a str, &'a str>,
  state: &'a State<'a>,
  translator: &'a Translator<'a>
) -> String {  
  let mut content = layouts
    .iter()
    .fold(
      content, 
      |content, layout| match *layout {
        "main-layout" => MainLayout {
            content,
            query,
            navigation: query.contains_key("main-layout-navigation")
          }
          .render_once()
          .unwrap(),        
        _ => content
      }
    );
  
  if !ajax {
    content = DefaultLayout {
        state,
        content,
        lang,
        translator
      }
      .render_once()
      .unwrap()
  }

  return String::from(content);
}
