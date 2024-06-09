use crate::handler::function_handler;
use crate::utils::init_logger;
use lambda_runtime::{run, service_fn, Error};
use log::LevelFilter;

mod env;
mod handler;
mod model;
mod securityhub;
mod sns;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_logger(LevelFilter::Debug);
    run(service_fn(function_handler)).await
}
