use crate::{
    config::{AppConfig, APP_CONFIG},
    handler,
    message::request::WsRequestMessage,
};
use aws_config::{Region, SdkConfig};
use aws_sdk_apigatewaymanagement::config;
use lambda_http::{
    request::RequestContext,
    service_fn,
    tracing::{self},
    Body, Error, Request, RequestExt, Response,
};
use serde_json::json;

pub async fn main() {
    std::env::set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
    lambda_http::tracing::init_default_subscriber();
    AppConfig::init().await;

    let shared_config = aws_config::from_env()
        .region(Region::new("ap-northeast-2"))
        .load()
        .await;
    let api_gw_client = make_gw_client(&shared_config);
    let dynamo_client = make_dynamo_client(&shared_config);
    let sqs_client = make_sqs_client(&shared_config);
    let http_client = make_http_client();

    if let Err(e) = lambda_http::run(service_fn(|event| async {
        request_handler(
            event,
            &api_gw_client,
            &dynamo_client,
            &sqs_client,
            &http_client,
        )
        .await
    }))
    .await
    {
        tracing::error!("error: {e:?}");
    }
}
fn make_http_client() -> reqwest::Client {
    reqwest::Client::new()
}
fn make_sqs_client(shared_config: &SdkConfig) -> aws_sdk_sqs::Client {
    aws_sdk_sqs::Client::new(shared_config)
}
fn make_dynamo_client(shared_config: &SdkConfig) -> aws_sdk_dynamodb::Client {
    aws_sdk_dynamodb::Client::new(shared_config)
}
fn make_gw_client(shared_config: &SdkConfig) -> aws_sdk_apigatewaymanagement::Client {
    let connection_url = APP_CONFIG
        .get()
        .unwrap()
        .settings
        .gw_ws
        .connections_url
        .as_str();

    let api_management_config = config::Builder::from(shared_config)
        .endpoint_url(connection_url)
        .build();

    let client = aws_sdk_apigatewaymanagement::Client::from_conf(api_management_config);
    client
}

async fn request_handler(
    request: Request,
    api_gw_client: &aws_sdk_apigatewaymanagement::Client,
    dynamo_client: &aws_sdk_dynamodb::Client,
    sqs_client: &aws_sdk_sqs::Client,
    http_client: &reqwest::Client,
) -> Result<Response<Body>, Error> {
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

    let Some(connection_id) = &ws.connection_id else {
        tracing::warn!("default connection_id empty");
        return Ok(Response::builder()
            .status(400)
            .body(Body::Text(json!({ "status": "disconnect" }).to_string()))?);
    };

    let body = request.body();
    match route_key.as_str() {
        "$connect" => {
            let b = handler::connect::ws_connect(
                dynamo_client,
                request.query_string_parameters(),
                connection_id,
                http_client,
            )
            .await?;

            if !b {
                return Ok(Response::builder()
                    .status(400)
                    .body(Body::Text(json!({ "status": "connect fail" }).to_string()))?);
            }

            return Ok(Response::builder()
                .status(200)
                .body(Body::Text(json!({ "status": "connected" }).to_string()))?);
        }
        "$disconnect" => {
            handler::disconnect::ws_disconnect(dynamo_client, connection_id).await?;
            return Ok(Response::builder()
                .status(200)
                .body(Body::Text(json!({ "status": "disconnect" }).to_string()))?);
        }
        "$default" => {
            match serde_json::from_slice::<WsRequestMessage>(&body) {
                Ok(body) => match body {
                    WsRequestMessage::Echo { msg } => {
                        handler::echo::echo_handler(api_gw_client, sqs_client, connection_id, msg)
                            .await?;
                    }
                    WsRequestMessage::TopicSubscribe { topic } => {
                        handler::topic::topic_subscribe(dynamo_client, connection_id, &topic)
                            .await?;
                    }
                    WsRequestMessage::TopicUnSubscribe { topic } => {
                        handler::topic::topic_unsubscribe(dynamo_client, connection_id, &topic)
                            .await?;
                    }
                    WsRequestMessage::RoomChat { msg, room_id } => {
                        handler::room::room_chat(
                            dynamo_client,
                            api_gw_client,
                            &connection_id,
                            &msg,
                            &room_id,
                        )
                        .await?;
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
