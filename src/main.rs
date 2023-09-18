mod router;

use std::{
    net::SocketAddr,
    process::{exit, Child, Command, Stdio},
    sync::{Arc, Mutex},
};

use axum::{
    body::{boxed, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
    response::Redirect,
    routing::get,
    Router,
};
use tower::ServiceExt;
use tower_http::services::ServeDir;

#[cfg(debug_assertions)]
fn initialise() -> Child {
    Command::new(env!("TC_PACKAGE_MANAGER"))
        .args(["run", "dev"])
        .current_dir(env!("TC_FRONTEND_DIR"))
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("could not start package manager")
}

#[cfg(not(debug_assertions))]
fn initialise() -> Child {
    unreachable!()
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    match ServeDir::new(format!("{}/dist", env!("TC_FRONTEND_DIR")))
        .oneshot(req)
        .await
    {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}

pub async fn file_handler(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let res = get_static_file(uri.clone()).await?;

    if res.status() == StatusCode::NOT_FOUND {
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = if cfg!(debug_assertions) {
        Router::new().fallback(get(|| async { Redirect::to("localhost:8080") }))
    } else {
        Router::new().fallback(get(file_handler))
    };

    let router = Router::new();

    let app = app.merge(router);

    if cfg!(debug_assertions) {
        let frontend = Arc::new(Mutex::new(initialise()));
        let frontend_pid = frontend
            .lock()
            .expect("could not get lock on child process")
            .id();

        let frontend_clone = frontend.clone();

        ctrlc::set_handler(move || {
            tracing::info!("Recieved CTRL+C signal, terminating...");

            tracing::debug!("Terminating front-end server...");
            frontend_clone
                .lock()
                .expect("could not get lock on child process")
                .kill()
                .unwrap_or_else(|_| panic!("could not automatically kill child frontend server process with id: {frontend_pid}"));
            tracing::debug!("... Done");

            exit(0);
        })
        .expect("could not set CTRL+C handler");

        tracing::debug!("Front-end process id: {frontend_pid}");
        tracing::debug!("Front-end web server listening on 'http://localhost:8080'")
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("could not bind server to {addr}"));
}
