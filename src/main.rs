#![warn(clippy::pedantic)]

mod api;
mod error;
mod prelude;
mod router;
mod utils;
mod ws;

use crate::{
	prelude::*,
	router::get_router,
	utils::{apidoc::ApiDoc, arctex::ArcTex, peer_map::PeerMap, W},
};

#[cfg(debug_assertions)]
use std::process::{Command, Stdio};
use std::{
	net::SocketAddr,
	process::{exit, Child},
};

use axum::{
	body::{boxed, Body},
	http::{HeaderMap, Request, StatusCode, Uri},
	response::Response,
	routing::get,
	Router,
};
use tokio::net::TcpListener;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

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
	unreachable!("should never be reached in release mode builds")
}

#[axum_macros::debug_handler]
async fn forward_requests(request: Request<Body>) -> Result<Response> {
	let client = reqwest::Client::new();

	let uri: reqwest::Url = W(request.uri()).try_into()?;
	let forward = reqwest::Request::new(request.method().to_owned(), uri);

	let res = client.execute(forward).await?;

	let status = res.status();
	let headers = res.headers().clone();
	let body = boxed(res.text().await?);

	let mut response = Response::builder().status(status);

	if response.headers_ref().is_none() {
		*response.headers_mut().unwrap() = HeaderMap::new();
	}
	for (key, value) in &headers {
		response.headers_mut().unwrap().append(key, value.clone());
	}

	response.body(body).map_err(Error::from)
}

#[axum_macros::debug_handler]
async fn get_static_file(uri: Uri) -> Result<Response> {
	let request = Request::builder()
		.uri(uri)
		.body(Body::empty())
		.map_err(Error::from)?;

	match ServeDir::new(fmt!("{}/dist", env!("TC_FRONTEND_DIR")))
		.oneshot(request)
		.await
	{
		Ok(response) => Ok(response.map(boxed)),
		Err(_) => unreachable!("error type is Infallible"),
	}
}

#[axum_macros::debug_handler]
async fn file_handler(uri: Uri) -> Result<Response> {
	let res = get_static_file(uri.clone()).await?;

	if res.status() == StatusCode::NOT_FOUND {
		match fmt!("{uri}.html").parse() {
			Ok(uri_html) => get_static_file(uri_html).await,
			Err(err) => Err(Error::ResponseError(
				StatusCode::INTERNAL_SERVER_ERROR,
				err.to_string(),
			)),
		}
	} else {
		Ok(res)
	}
}

// The main function can change depending on the mode that the application was
// compiled in.
//
// If the application was compiled in debug mode:
// - The application starts a vite server as a child process, running the front-end
// - It then forwards every request made to itself to the vite server
// - It spins up the websocket server, so that the front-end can get the logs and anything
//   else that may be sent through the pipe via websockets.
//
// If the application was compiled in release mode:
// - The application serves the static files compiled by vite
// - It spins up the websocket server, so that the front-end can get the logs and anything
//   else that may be sent through the pipe via websockets.
#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();
	tracing_subscriber::fmt::init();

	let app = if cfg!(debug_assertions) {
		// if we're in debug, we have to forward the requests made by the client
		// connecting to us, to the vite server that is being ran in the background.
		Router::new().fallback(get(forward_requests))
	} else {
		// if we're in release, we can serve the static files directly, without needing
		// to run vite in the background.
		Router::new().fallback(get(file_handler))
	};

	// we can use this as an endpoint for any additional actions the front-end may
	// need besides just receiving logs.
	let router = Router::new()
		.nest("/api", get_router())
		.merge(Redoc::with_url("/docs", ApiDoc::openapi()));
	let app = app.merge(router);

	// this only applies to debug builds - everything here should be compiled
	// out/ignored completely during release runtime.
	if cfg!(debug_assertions) {
		// we need to get a lock on the vite process, so we can safefully shut it down
		// after this server has been closed - we don't want random zombie processes
		// that cannot be stopped.
		let frontend = ArcTex::new(initialise());
		let frontend_pid = frontend.lock().id(); // we get the pid so that the vite server can be manually killed if need be

		let frontend_clone = frontend.clone();

		ctrlc::set_handler(move || {
			tracing::info!("Recieved CTRL+C signal, terminating...");

			tracing::debug!("Terminating front-end server...");
			frontend_clone.lock().kill().unwrap_or_else(|_| {
				panic!("could not automatically kill child frontend server process with id: {frontend_pid}")
			});
			tracing::debug!("... Done");

			exit(0); // we exit the main program gracefully
		})
		.expect("could not set CTRL+C handler");

		tracing::debug!("Vite process id: {frontend_pid}");
		tracing::debug!("Vite server reachable on 'http://localhost:8080'");
	}

	// bind the front-end to :3000
	let listening_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	tracing::info!("Listening on http://{listening_addr}");
	tracing::info!("API documentation available at http://{listening_addr}/docs");
	tokio::spawn(axum::Server::bind(&listening_addr).serve(app.into_make_service()));

	// bind the websocket server to :3001
	let websocket_addr = SocketAddr::from(([127, 0, 0, 1], 3001));
	let ws_socket = TcpListener::bind(&websocket_addr)
		.await
		.unwrap_or_else(|err| {
			panic!("An error occurred when binding websockets to port: {err}")
		});
	tracing::debug!("Websockets for frontend listening on ws://{websocket_addr}");

	let peers = PeerMap::new();
	while let Ok((raw_stream, addr)) = ws_socket.accept().await {
		tokio::spawn(ws::handle_connection(peers.clone(), raw_stream, addr));
	}
}
