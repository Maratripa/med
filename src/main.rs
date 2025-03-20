#![warn(clippy::pedantic, clippy::perf, clippy::complexity, clippy::suspicious)]
#![allow(clippy::missing_errors_doc, clippy::should_implement_trait)]

use anyhow::Result;

mod application;
use application::Application;
mod cursor;
mod document;
mod editor;
mod terminal;
mod view;

fn main() -> Result<()> {
    let _exit_code = main_impl()?;
    Ok(())
}

#[tokio::main]
async fn main_impl() -> Result<i32> {
    let args: Vec<String> = std::env::args().collect();

    let mut app = Application::new(args);

    app.run().await?;

    Ok(0)
}
