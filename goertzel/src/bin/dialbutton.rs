use std::env;

use anyhow::{anyhow, Result};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{debug_handler, Router};
use rsip::prelude::{HeadersExt, ToTypedHeader};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{self, fmt, EnvFilter};

use goertzel::contacts::CONTACTS;
use goertzel::sip::{assert_status, tlssocket, SERVER_NAME, SERVER_PORT};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route(
            "/healthcheck",
            get(|| async {
                info!("thx for checkin in");
                "healthy"
            }),
        )
        .route("/dial", post(post_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn post_handler() -> Result<&'static str, AppError> {
    info!("getting public ip");
    let ip = public_ip::addr_v4().await.ok_or(anyhow!("no ip"))?;

    info!("getting tls conn");
    let tls_conn = tlssocket::TlsSipConn::new(ip, SERVER_NAME, SERVER_PORT).await?;

    let password = env::var("SIP_PASSWORD")?;
    let mut dialog = tls_conn.dialog(String::from("1103")).await;
    dialog.register(password.clone()).await?;

    let mut dialog = tls_conn.dialog(String::from("1103")).await;
    let to = (*CONTACTS)
        .get("1102")
        .ok_or(anyhow!("contact is missing after I EXPLICITLY checked it"))?;
    dialog.invite(password.clone(), to.clone()).await?;

    let resp_200 = dialog.recv().await?;
    assert_status(&resp_200.clone().try_into()?)?;
    dialog.set_to(resp_200.to_header()?.typed()?);

    let msg = dialog.recv().await?;
    dialog.ack(msg.try_into()?).await?;

    let invite = dialog.recv().await?;
    let sdp = dialog.sdp_from(invite.clone().try_into()?)?;
    let ref resp = dialog.sdp_response_to(invite.clone().try_into()?, rsip::StatusCode::OK, sdp)?;
    dialog.send(resp.clone()).await?;

    let _ack = dialog.recv().await?;

    Ok("yay")
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
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
