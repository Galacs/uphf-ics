use actix_web::{get, http::header::Header, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::headers::{
    authorization::{self, Basic},
    www_authenticate,
};
use snafu::prelude::*;

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

#[cfg_attr(feature = "instrument", tracing::instrument)]
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
        return Ok(HttpResponse::Unauthorized()
            .insert_header(www_authenticate::WwwAuthenticate(challenge))
            .finish());
    };
    let (username, password) = (
        creds.as_ref().user_id(),
        creds.as_ref().password().context(NoPassword)?,
    );
    Ok(HttpResponse::Ok().body(download_ical(username, password).await?))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(feature = "instrument")]
    {
        use opentelemetry::trace::TracerProvider;
        use opentelemetry_otlp::WithExportConfig;
        use tracing_subscriber::{fmt, prelude::*, EnvFilter};

        let otlp_exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(std::env::var("UPHF_ICS_OTLP_ENDPOINT").unwrap_or_default());
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(otlp_exporter)
            .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
                opentelemetry_sdk::Resource::new([opentelemetry::KeyValue::new(
                    "service.name",
                    "uphf-ics",
                )]),
            ))
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .unwrap()
            .tracer("uphf-ics");

        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(fmt::layer().pretty())
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .init();
    }

    HttpServer::new(|| {
        let app = App::new().service(hello);
        #[cfg(feature = "instrument")]
        let app = app.wrap(tracing_actix_web::TracingLogger::default());
        app
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
