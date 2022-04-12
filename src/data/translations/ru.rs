use std::collections::HashMap;

use azizka_rust_i18n::{DataOptions, Value};

pub fn get_data<'a>() -> DataOptions<'a> {
  DataOptions { 
    values: Some(
      HashMap::from([
        ("My Routes", Value::Single("Мои пути")),
        ("Sign In", Value::Single("Войти")),
        ("Sign Up", Value::Single("Регистрация")),
        ("Sign In/Up", Value::Single("Войти или Зарегистрироваться")),
        ("Sign Out", Value::Single("Выйти")),
        ("Name", Value::Single("Имя")),
        ("Password", Value::Single("Пароль")),
        ("Photo", Value::Single("Фото")),
        ("Cancel", Value::Single("Отмена")),
        ("Or use the service", Value::Single("Или используйте сервис")),
        ("Auth service", Value::Single("Сервис аутентификации")),
        ("User with this email and password doesn't exist", Value::Single("Пользователь с таким email и паролем не найден")),
        ("User with this email already exists", Value::Single("Пользователь с таким email уже существует")),
        ("Email required", Value::Single("Необходимо заполнить email")),
        ("Name required", Value::Single("Необходимо заполнить имя")),
        ("Password required", Value::Single("Необходимо заполнить пароль")),
        ("To link with this OAuth account need to Sign Up", Value::Single("Чтобы связать этого OAuth пользователя необходимо зарегитрироваться")),
        ("Could not to Sign In with this OAuth service", Value::Single("Не удалось войти с помощью этого OAuth сервиса"))
      ])
    ), 
    contexts: None 
  }
}
