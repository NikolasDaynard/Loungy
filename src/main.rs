use app::run_app;
use gpui::App;

mod app;
mod assets;
mod commands;
mod components;
mod db;
mod hotkey;
mod paths;
mod platform;
mod query;
mod state;
mod theme;
mod window;
mod workspace;

#[async_std::main]
async fn main() {
    env_logger::init();
    let app = App::new();

    run_app(app)
}
