use std::collections::HashMap;

use azizka_rust_i18n::Translator;
use sailfish::TemplateOnce;

use super::State;

#[derive(TemplateOnce)]
#[template(path = "layouts/default-layout.html")]
pub struct DefaultLayout<'a> 
{
  pub state: &'a State<'a>,
  pub lang: &'a str,
  pub content: String,
  pub translator: &'a Translator<'a>
}

#[derive(TemplateOnce)]
#[template(path = "layouts/main-layout.html")]
pub struct MainLayout<'a> {
  pub navigation: bool,
  pub content: String,
  pub query: &'a HashMap<&'a str, &'a str>
}

