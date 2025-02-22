pub mod connect;
pub mod default;
pub mod disconnect;
pub mod echo;
pub mod room;
pub mod topic;

pub fn get_sub_by_connection_id(
    dynamo_client: &aws_sdk_dynamodb::Client,
    connection_id: &str,
) -> Option<String> {
    None
}
