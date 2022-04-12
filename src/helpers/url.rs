use std::collections::HashMap;

pub fn get_query_parameters(
  query: &HashMap<String, String>
) -> String {
  let mut params: Vec<String> = Vec::new();

  for (key, value) in query {
    if value.is_empty() {
      params.push(
        key.clone()
      );
    } else {
      params.push(
        format!("{key}={value}")
      );
    }
  }

  return params.join("&");
}

pub fn set_query_parameter(
  query: &HashMap<String, String>,
  key: &str,
  value: &str
) -> String {
  let mut data = query.clone();

  data.insert(
    String::from(key), 
    String::from(value)
  );

  return get_query_parameters(&data);
}

pub fn toggle_query_parameter(
  query: &HashMap<String, String>,
  param: &str
) -> String {
  let mut data = query.clone();

  if data.contains_key(param) {
    data.remove(param);

    return get_query_parameters(
      &data
    );
  }

  return set_query_parameter(
    &data, 
    param, 
    &String::from("1")
  );
}
