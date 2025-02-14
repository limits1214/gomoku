use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RoomInfo {
    pub room_name: String,
    pub room_id: String,
    pub room_num: String,
    pub channel: String,
}
