use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, UNIX_EPOCH};

use axum::extract::{FromRequestParts, Path, Query};
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::Html;
use axum::routing::delete;
use axum::{Router, routing::get};
use rinja::Template;
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::log::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    init_tracing();

    let app = Application {
        host: "127.0.0.1".to_string(),
        port: 3000,
    };

    let router = Router::new()
        .route("/", get(index))
        .route("/files/{directory}", get(get_file_list))
        .route("/foo", get(foo_handler))
        .route("/video", get(video_handler))
        .route("/delete", delete(delete_file))
        .route("/sort", get(sort_files))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .nest_service("/resources", ServeDir::new("resources"));

    let listener = TcpListener::bind(&app.address()).await.unwrap();

    debug!("Listening on {}", &app.address());

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

fn init_tracing() {
    let tracing = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer());

    if let Err(err) = tracing.try_init() {
        eprintln!("Exiting. Error occurred while initializing tracing: {err}");
        std::process::exit(1);
    }
}

struct Application {
    host: String,
    port: u16,
}

pub type Files = Vec<File>;

#[derive(Debug)]
struct File {
    name: String,
    modified: u64,
    size: u64,
    path: String,
    is_directory: bool, // nechat ate se neplete s path
}

impl Application {
    fn address(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.host, self.port)).unwrap()
    }
}

#[derive(Deserialize)]
struct IndexQuery {
    #[serde(rename = "path")]
    directory: Option<String>,
}
async fn index(Query(query): Query<IndexQuery>) -> Result<Html<String>, ()> {
    #[derive(Template, Debug)]
    #[template(path = "index.html")]
    struct Tmpl {
        lang: String,
        files: Files,
    }

    let files = if query.directory.is_some() {
        get_files(query.directory.unwrap())
    } else {
        get_files(".".to_string())
    };

    let template = Tmpl {
        lang: "en".to_string(),
        files,
    };

    Ok(Html(template.render().unwrap()))
}

struct SortingExtractor;

mod filters {
    use humansize::DECIMAL;
    use std::str::FromStr;

    pub fn format_size<T: std::fmt::Display>(s: T) -> ::rinja::Result<String> {
        let size = usize::from_str(&s.to_string()).unwrap();
        Ok(humansize::format_size(size, DECIMAL))
    }
}


#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum DefaultSortType {
    Unix,
    Windows,
}

fn sort(mut files: Files, sorting: Sorting) -> Files {
    match sorting {
        Sorting::Name(order) => {
            files.sort_by(|a, b| match order {
                SortingType::Ascending => a.name.cmp(&b.name),
                SortingType::Descending => b.name.cmp(&a.name),
            });
        }
        Sorting::Size(order) => {
            files.sort_by(|a, b| match order {
                SortingType::Ascending => a.size.cmp(&b.size),
                SortingType::Descending => b.size.cmp(&a.size),
            });
        }
        Sorting::Modified(order) => {
            files.sort_by(|a, b| match order {
                SortingType::Ascending => a.modified.cmp(&b.modified),
                SortingType::Descending => b.modified.cmp(&a.modified),
            });
        }
        _ => {}
    }
    files
}

fn get_files(directory: String) -> Files {
    let mut files = Vec::new();

    for entry in std::fs::read_dir(&format!("./{directory}")).unwrap() {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        let path = if entry.path().parent().unwrap().to_str() == Some(".") {
            "".to_string()
        } else {
            entry.path().parent().unwrap().to_str().unwrap().to_string()
        };

        let file = File {
            name: entry
                .file_name()
                .to_os_string()
                .to_string_lossy()
                .to_string(),
            modified: metadata
                .modified()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            size: metadata.len(),
            is_directory: entry.file_type().unwrap().is_dir(),
            path,
        };

        files.push(file);
    }

    files
}

async fn get_file_list(Path(directory): Path<String>) -> Html<String> {
    #[derive(Template, Debug)]
    #[template(path = "file-list.html")]
    struct Tmpl {
        files: Files,
    }

    let template = Tmpl {
        files: sort(get_files(directory), Sorting::Name(SortingType::Descending)),
    };

    Html(template.render().unwrap())
}

async fn foo_handler() -> Html<&'static str> {
    Html("<p>foo</p>")
}

async fn video_handler() -> Html<&'static str> {
    let html = r##"
    <video width="640" height="360" controls autoplay muted>
    <source src="/resources/video.mp4" type="video/mp4">
    Your browser does not support the video tag.
    </video>
"##;
    Html(html)
}

async fn delete_file(path: PathRequest) {
    tokio::fs::remove_file(path.full_path).await.unwrap()
}

struct PathRequest {
    directory: PathBuf,
    file: PathBuf,
    full_path: PathBuf,
}

impl<S> FromRequestParts<S> for PathRequest
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        #[derive(serde::Deserialize)]
        struct FilePath {
            #[serde(rename = "path")]
            directory: PathBuf,
            file: PathBuf,
        }

        let mut query = Query::<FilePath>::from_request_parts(parts, state).await;

        let Ok(mut query) = query else {
            return Err((StatusCode::BAD_REQUEST, "bad request"));
        };

        if query.directory == PathBuf::from(".") || query.directory == PathBuf::from("/") {
            query.directory = PathBuf::from("")
        }

        Ok(PathRequest {
            file: query.file.clone(),
            directory: query.directory.clone(),
            full_path: query.directory.join(&query.file),
        })
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum SortingType {
    Ascending,
    Descending,
}


#[derive(Deserialize)]
struct SortQuery {
    #[serde(rename = "mode", deserialize_with = "deserialize_sorting")]
    sorting: Sorting,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum Sorting {
    Default(DefaultSortType),
    Name(SortingType),
    Size(SortingType),
    Modified(SortingType),
}

async fn sort_files(Query(query): Query<SortQuery>) ->Html<String> {
    let files = get_files(".".to_string());
    let files = sort(files, query.sorting);

    #[derive(Template, Debug)]
    #[template(path = "file-list.html")]
    struct Tmpl {
        files: Files,
    }

    let template = Tmpl {
        files,
    };

    Html(template.render().unwrap())
}

fn deserialize_sorting<'de, D>(deserializer: D) -> Result<Sorting, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let mut parts = s.split('.');

    match (parts.next(), parts.next()) {
        (Some("name"), Some("ascending")) => Ok(Sorting::Name(SortingType::Ascending)),
        (Some("name"), Some("descending")) => Ok(Sorting::Name(SortingType::Descending)),
        (Some("size"), Some("ascending")) => Ok(Sorting::Size(SortingType::Ascending)),
        (Some("size"), Some("descending")) => Ok(Sorting::Size(SortingType::Descending)),
        (Some("modified"), Some("ascending")) => Ok(Sorting::Modified(SortingType::Ascending)),
        (Some("modified"), Some("descending")) => Ok(Sorting::Modified(SortingType::Descending)),
        _ => Err(serde::de::Error::custom("Invalid sorting mode")),
    }
}