use crate::PathRequest;

pub async fn delete_file(path: PathRequest) {
    tokio::fs::remove_file(path.full_path).await.unwrap()
}
