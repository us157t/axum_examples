use axum::{response::Html, routing::get, Router};
use listenfd::ListenFd;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
	let app = Router::new()
		.route("/", get(h));
	let mut lf = ListenFd::from_env();
	let lis = match lf.take_tcp_listener(0).unwrap() {
		Some(lis) => {
			lis.set_nonblocking(true).unwrap();
			TcpListener::from_std(lis).unwrap()
		}
		None => TcpListener::bind("127.0.0.1:3000")
		.await.unwrap(),
	};
	println!("istening on {}", lis.local_addr().unwrap());
	axum::serve(lis, app).await.unwrap();
}

async fn h() -> Html<&'static str> {
	Html("<h1>Hello</h1>")
}
