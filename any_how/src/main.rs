use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	routing::get,
	Router,
};

fn y123() {}


#[tokio::main]
async fn main() {
	let app = Router::new()
		.route("/", get(h));

	let lis = tokio::net::TcpListener::bind(
		"127.0.0.1:3000").await.unwrap();

	axum::serve(lis, app).await.unwrap();
}

async fn h() -> Result<(), AppError> {
	try_thing()?;
	Ok(())
}

fn try_thing() -> Result<(), anyhow::Error> {
	anyhow::bail!("it failed!")
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
	fn into_response(self) -> Response {
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Something went wrong:{}", self.0),	
		).into_response()
	}
}

impl<E> From<E> for AppError
where
	E: Into<anyhow::Error>,
{
	fn from(err: E) -> Self {
		Self(err.into())
	}
}
