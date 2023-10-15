mod components;
mod constants;
mod pages;
mod styles;
mod database;
mod app;

use crate::app::App;

fn main() -> Result<(), anyhow::Error> {
	App::run()
}
