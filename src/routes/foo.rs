use axum::response::Html;

pub async fn foo_handler() -> Html<&'static str> {
    Html("<p>foo</p>")
}
