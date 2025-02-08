mod app;
mod config;
mod handler;
mod router;

pub async fn start_app() -> Result<(), lambda_http::Error> {
    use lambda_http::run;
    use lambda_http::tracing;

    std::env::set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
    tracing::init_default_subscriber();

    let app = app::create_app().await;
    run(app).await?;

    Ok(())
}
