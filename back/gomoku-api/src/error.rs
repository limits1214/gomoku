use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error(transparent)]
    Dynamo(#[from] aws_sdk_dynamodb::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
}
