#[async_std::test]
async fn test_default() -> tide::Result<()> {
  use crate::{data, routes};

  use tide::{
    StatusCode, 
    http::mime::{HTML, JSON}
  };
  
  use tide_testing::TideTestingExt;

  let mut app = tide::with_state(data::State::new());

  routes::home::register_routes(&mut app);

  let res = app.get("/").await?;

  assert_eq!(StatusCode::Ok, res.status());
  assert_eq!(Some(HTML), res.content_type());

  let res = app.get("/?ajax=1").await?;

  assert_eq!(StatusCode::Ok, res.status());
  assert_eq!(Some(JSON), res.content_type());

  let res = app.get("/?ajax=1&init=1").await?;

  assert_eq!(StatusCode::Ok, res.status());
  assert_eq!(Some(HTML), res.content_type());

  let res = app.get("/123").await?;

  assert_eq!(StatusCode::NotFound, res.status());

  Ok(())
}
