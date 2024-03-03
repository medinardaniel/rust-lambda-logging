use aws_sdk_xray as xray;
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::Value;
use log::{self};

#[::tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let func = service_fn(function_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    log::info!("Processing event: {:?}", event);
    
    let config = aws_config::load_from_env().await;
    let _xray_client = xray::Client::new(&config);

    // Add your function logic and any X-Ray annotations here

    Ok(event)
}