use axum::Router;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::{config::app_state::ArcAppState, controller::test::TestApi};

pub fn openapi_route(_state: ArcAppState) -> Router<ArcAppState> {
    Router::new().merge(SwaggerUi::new("/").url("/api-docs/openapi.json", ApiDoc::openapi()))
}

#[derive(OpenApi)]
#[openapi(
    nest(
        (path="/test", api = TestApi),
        // (path="/api/user", api = UserApi),
        // (path="/api/auth", api = AuthApi)
    ),
    modifiers(&SecurityAddon),
    security(
        (), // <-- make optional authentication
        ("access_token" = [])
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "access_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
