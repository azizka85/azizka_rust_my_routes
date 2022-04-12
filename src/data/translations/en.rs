use std::collections::HashMap;

use azizka_rust_i18n::{DataOptions, Value};

pub fn get_data<'a>() -> DataOptions<'a> {
  DataOptions { 
    values: Some(
      HashMap::from([
        ("My Routes", Value::Single("My Routes")),
        ("Sign In", Value::Single("Sign In")),
        ("Sign Up", Value::Single("Sign Up")),
        ("Sign In/Up", Value::Single("Sign In/Up")),
        ("Sign Out", Value::Single("Sign Out")),
        ("Name", Value::Single("Name")),
        ("Password", Value::Single("Password")),
        ("Photo", Value::Single("Photo")),
        ("Cancel", Value::Single("Cancel")),
        ("Or use the service", Value::Single("Or use the service")),
        ("Auth service", Value::Single("Auth service")),
        ("User with this email and password doesn't exist", Value::Single("User with this email and password doesn't exist")),
        ("User with this email already exists", Value::Single("User with this email already exists")),
        ("Email required", Value::Single("Email required")),
        ("Name required", Value::Single("Name required")),
        ("Password required", Value::Single("Password required")),
        ("To link with this OAuth account need to Sign Up", Value::Single("To link with this OAuth account need to Sign Up")),
        ("Could not to Sign In with this OAuth service", Value::Single("Could not to Sign In with this OAuth service"))
      ])
    ), 
    contexts: None 
  }
}
