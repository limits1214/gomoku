use aws_config::Region;
use aws_sdk_apigatewaymanagement::config;
use gomoku_ws::{config::AppConfig, handler, message::request::WsRequestMessage};
use lambda_http::{request::RequestContext, service_fn, tracing, Body, Error, Request, Response};
use serde_json::json;

#[tokio::main]
async fn main() {
    std::env::set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
    lambda_http::tracing::init_default_subscriber();
    AppConfig::init().await;

    if let Err(e) = lambda_http::run(service_fn(handler)).await {
        tracing::error!("error: {e:?}");
    }
}

async fn handler(request: Request) -> Result<Response<Body>, Error> {
    // tracing::info!("qs: {:?}", request.query_string_parameters());
    // tracing::info!("headers: {:?}", request.headers());

    let context = request
        .extensions()
        .get::<lambda_http::request::RequestContext>();
    let Some(RequestContext::WebSocket(ws)) = context else {
        return Ok(Response::builder().status(400).body(Body::Text(
            json!({ "error": "Not Websocket Context" }).to_string(),
        ))?);
    };

    let Some(route_key) = &ws.route_key else {
        return Ok(Response::builder()
            .status(400)
            .body(Body::Text(json!({ "error": "No Route Key" }).to_string()))?);
    };
    // tracing::info!("route_key: {:?}", route_key);
    let api_gw_client = make_gw_client().await;

    let http_client = reqwest::Client::new();

    let body = request.body();

    match route_key.as_str() {
        "$connect" => {
            return Ok(Response::builder()
                .status(200)
                .body(Body::Text(json!({ "status": "connected" }).to_string()))?);
        }
        "$disconnect" => {
            return Ok(Response::builder()
                .status(200)
                .body(Body::Text(json!({ "status": "disconnect" }).to_string()))?);
        }
        "$default" => {
            let Some(connection_id) = &ws.connection_id else {
                tracing::warn!("connection_id empty");
                return Ok(Response::builder()
                    .status(400)
                    .body(Body::Text(json!({ "status": "disconnect" }).to_string()))?);
            };
            match serde_json::from_slice::<WsRequestMessage>(&body) {
                Ok(body) => match body {
                    WsRequestMessage::Echo { msg } => {
                        handler::echo::echo_handler(api_gw_client, connection_id, msg).await?;
                    }
                    WsRequestMessage::WsInitial { jwt } => {
                        //
                    }
                },
                Err(err) => {
                    tracing::error!("$default from_slice err: {err}");
                }
            }

            return Ok(Response::builder()
                .status(200)
                .body(Body::Text(json!({ "status": "default" }).to_string()))?);
        }
        _ => {
            tracing::warn!("not matched route key, {:?}", route_key);
        }
    }
    Ok(Response::builder()
        .status(400)
        .body(Body::Text(json!({ "error": "Unknown route" }).to_string()))?)
}

async fn make_gw_client() -> aws_sdk_apigatewaymanagement::Client {
    let shared_config = aws_config::from_env()
        .region(Region::new("ap-northeast-2"))
        .load()
        .await;

    let api_management_config = config::Builder::from(&shared_config)
        .endpoint_url("https://0gnlyzkqd6.execute-api.ap-northeast-2.amazonaws.com/dev")
        .build();
    let client = aws_sdk_apigatewaymanagement::Client::from_conf(api_management_config);
    client
}
