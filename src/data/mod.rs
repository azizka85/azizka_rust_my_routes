use std::collections::HashMap;

use azizka_rust_i18n::Translator;
use serde::{Serialize, Deserialize};

use self::translations::{kz, ru, en};

pub mod components;
pub mod layouts;
pub mod pages;
pub mod translations;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct State<'a> {
  pub default_language: &'a str,
  pub languages: HashMap<&'a str, LanguageInfo<'a>>
}

impl<'a> State<'a> {
  pub fn new() -> Self {
    State { 
      default_language: "kz", 
      languages: HashMap::from([
        ("kz", LanguageInfo {
          image: "/images/flags/kz.svg",
          label: "Қазақша",
          translator: Translator::create(&kz::get_data())
        }),
        ("ru", LanguageInfo {
          image: "/images/flags/ru.svg",
          label: "Русский",
          translator: Translator::create(&ru::get_data())
        }),
        ("en", LanguageInfo {
          image: "/images/flags/en.svg",
          label: "English",
          translator: Translator::create(&en::get_data())
        })
      ]) 
    }
  }    
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LanguageInfo<'a> {
  pub image: &'a str,
  pub label: &'a str,
  pub translator: Translator<'a>
}
