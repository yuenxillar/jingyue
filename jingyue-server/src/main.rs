use application::{Applictaion, JingyueApplication};
use clap::Parser;

mod application;
mod config;
mod error;
mod handler;
mod model;
mod response;
mod state;
mod support;
mod util;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub(crate) struct ApplicationArgs {
    #[arg(long)]
    pub config_location: Option<String>,
}

impl Default for ApplicationArgs {
    fn default() -> Self {
        Self::parse()
    }
}

#[tokio::main]
async fn main() {
    JingyueApplication::run().await.unwrap();
}
