use std::collections::HashMap;

use azizka_rust_i18n::{DataOptions, Value};

pub fn get_data<'a>() -> DataOptions<'a> {
  DataOptions { 
    values: Some(
      HashMap::from([
        ("My Routes", Value::Single("Менің жолдарым")),
        ("Sign In", Value::Single("Кіру")),
        ("Sign Up", Value::Single("Тіркелу")),
        ("Sign In/Up", Value::Single("Кіру немесе Тіркелу")),
        ("Sign Out", Value::Single("Шығу")),
        ("Name", Value::Single("Аты")),
        ("Password", Value::Single("Пароль")),
        ("Photo", Value::Single("Фото")),
        ("Cancel", Value::Single("Болдырмау")),
        ("Or use the service", Value::Single("Немесе сервисті қолдаңыз")),
        ("Auth service", Value::Single("Аутентификация сервисі")),
        ("User with this email and password doesn't exist", Value::Single("Қолданушы осындай email және парольмен табылған жоқ")),
        ("User with this email already exists", Value::Single("Қолданушы осындай email бар")),
        ("Email required", Value::Single("Email толтыру керек")),
        ("Name required", Value::Single("Атыңды толтыру керек")),
        ("Password required", Value::Single("Парольды толтыру керек")),
        ("To link with this OAuth account need to Sign Up", Value::Single("Мына OAuth аккантты байланыстыру үшін тіркелу керек")),
        ("Could not to Sign In with this OAuth service", Value::Single("Мына OAuth сервис арқылы кіре алмады"))
      ])
    ), 
    contexts: None 
  }
}
