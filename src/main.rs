#![warn(clippy::all, clippy::pedantic)]
use tokio::runtime::Runtime;
use tuitask::{app::App, logger};

fn main() -> color_eyre::Result<()> {
    logger::init("logs/app.log")?;
    logger::info("Application started")?;

    color_eyre::install()?;
    dotenvy::dotenv().ok();

    let runtime = Runtime::new()?;
    let handle = runtime.handle().clone();

    let terminal = ratatui::init();
    let result = App::new(handle).run(terminal);

    if let Err(ref error) = result {
        let _ = logger::error(format!("Application error: {error}"));
    } else {
        let _ = logger::info("Application exited successfully");
    }
    runtime.shutdown_background();
    ratatui::restore();

    result
}
