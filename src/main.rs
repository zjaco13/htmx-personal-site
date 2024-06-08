use axum::{extract::Extension, http::StatusCode, response::Html, routing::get, Router};
use std::sync::Arc;
use tera::{Context, Tera};
use tower::ServiceBuilder;

async fn index_handler(tera: Extension<Arc<Tera>>) -> Result<Html<String>, (StatusCode, String)> {
    let context = Context::new();
    match tera.render("index.html", &context) {
        Ok(rendered) => Ok(Html(rendered)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Tera
    let tera = Tera::new("templates/**/*.html")
        .map_err(|err| format!("Template parsing error: {}", err))?;

    // Build our application with a route
    let app = Router::new()
        .route("/", get(index_handler))
        .layer(ServiceBuilder::new().layer(Extension(Arc::new(tera))));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    // Bind and serve the application
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
