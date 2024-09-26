use actix_web::{get, http::header::Header, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::headers::{
    authorization::{self, Basic},
    www_authenticate,
};
use snafu::prelude::*;
use tracing_actix_web::TracingLogger;

use uphf_edt::*;

#[derive(Debug, Snafu)]
pub enum IcsError {
    #[snafu(context(true), display("no password provided"))]
    NoPassword,
    #[snafu(context(false), display("Auth error: {source}"))]
    Auth { source: uphf_auth::AuthError },
    #[snafu(context(false), display("Edt error: {source}"))]
    Edt { source: uphf_edt::EdtError },
}
impl actix_web::error::ResponseError for IcsError {}

async fn download_ical(username: &str, password: &str) -> Result<String, IcsError> {
    let execution_value = uphf_auth::get_new_cas_execution_value().await?;

    let cookie = uphf_auth::get_cas_tgc_cookie(username, password, &execution_value).await?;
    let jsession = uphf_edt::get_edt_jsession_id(&cookie).await?;

    let body = get_edt_body(&jsession).await?;

    let jids = get_ical_export_jid(&body).await?;

    Ok(download_edt_ics_file(&jsession, &jids.0, &jids.1).await?)
}

#[get("/ics")]
async fn hello(req: HttpRequest) -> Result<impl Responder, IcsError> {
    // returns 401 with a www challenge if no http basic auth header is given
    let Ok(creds) = authorization::Authorization::<Basic>::parse(&req) else {
        let challenge = www_authenticate::basic::Basic::new();
        tracing::info!("request without any creds, sending www challenge");
        return Ok(HttpResponse::Unauthorized()
            .insert_header(www_authenticate::WwwAuthenticate(challenge))
            .finish());
    };
    let (username, password) = (
        creds.as_ref().user_id(),
        creds.as_ref().password().context(NoPassword)?,
    );
    tracing::info!("creds received, trying to download ical...");
    Ok(HttpResponse::Ok().body(download_ical(username, password).await?))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).unwrap();

    HttpServer::new(|| App::new().service(hello).wrap(TracingLogger::default()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
