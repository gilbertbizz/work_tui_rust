mod app;
mod components;
mod pages;
mod styles;

use crate::app::App;

fn main() -> anyhow::Result<()> {
    App::run_app()
}


