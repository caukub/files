use crate::PathRequest;
use crate::error::AppError;

pub async fn delete_file(path: PathRequest) -> Result<(), AppError> {
    tokio::fs::remove_file(path.full_path)
        .await
        .map_err(|_| AppError::Foo)?;

    Ok(())
}
