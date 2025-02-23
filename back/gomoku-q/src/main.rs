use aws_lambda_events::sqs::SqsEvent;
use lambda_runtime::tracing;
use lambda_runtime::Error;
use lambda_runtime::LambdaEvent;
use serde_json::Value;
#[tokio::main]
async fn main() {
    lambda_runtime::tracing::init_default_subscriber();
    let res = lambda_runtime::run(lambda_runtime::service_fn(request_handler)).await;
    if let Err(err) = res {
        tracing::error!("err: {err:?} ");
    }
}
async fn request_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    for record in event.payload.records {
        if let Some(body) = &record.body {
            tracing::info!("body: {body:?}");
            let parsed: Value = serde_json::from_str(body)?;
            println!("Received SQS Message: {:?}", parsed);
        }
    }
    Ok(())
}
