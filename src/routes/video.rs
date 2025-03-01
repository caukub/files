use axum::response::Html;

pub async fn video_handler() -> Html<&'static str> {
    let html = r##"
    <video width="640" height="360" controls autoplay muted>
    <source src="/resources/video.mp4" type="video/mp4">
    Your browser does not support the video tag.
    </video>
"##;
    Html(html)
}
