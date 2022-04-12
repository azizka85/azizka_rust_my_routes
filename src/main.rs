mod data;
mod routes;
mod helpers;

use std::env;

#[async_std::main]
async fn main() -> tide::Result<()> {
	let port = env::var("PORT").unwrap_or(String::from("3000"));

	println!("Running on 0.0.0.0:{port}");

	let mut app = tide::with_state(data::State::new());	

	routes::home::register_routes(&mut app);

	app.listen(format!("0.0.0.0:{port}")).await?;

	Ok(())
}
