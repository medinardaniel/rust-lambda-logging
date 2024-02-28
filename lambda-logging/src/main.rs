use lambda_runtime::{service_fn, Error};
use serde_json::Value;
use aws_sdk_xray::{Client as XRayClient};

async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, context) = event.into_parts();
    let config = aws_config::load_from_env().await;
    let xray_client = XRayClient::new(&config);

    // Start an X-Ray segment
    let trace_header_str = ctx.xray_trace_id.as_deref().unwrap_or_default();
    xray_client.begin_segment("MyRustLambdaFunction", Some(trace_header_str), None).await?;

    log::info!("Processing event");
    // Your function logic goes here

    // End the X-Ray segment
    xray_client.end_segment("MyRustLambdaFunction").await?;

    Ok(event)
}
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Info).unwrap(); // Adjust the logging level as needed
    let func = service_fn(function_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}