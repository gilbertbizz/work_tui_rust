mod tabs;
mod colors;
mod root;
mod term;
mod theme;
mod constants;
mod app;

use app::App;
use anyhow::Result;

fn main() -> Result<()>{
    App::run()    
}
