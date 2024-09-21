use actix_web::{get, http::header::Header, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::headers::{
    authorization::{self, Basic},
    www_authenticate,
};

use uphf_edt::*;

async fn download_ical(username: &str, password: &str) -> String {
    let execution_value = uphf_auth::get_new_cas_execution_value().await;

    let cookie = uphf_auth::get_cas_tgc_cookie(username, password, &execution_value.unwrap()).await;
    let jsession = uphf_edt::get_edt_jsession_id(&cookie.unwrap()).await;

    let body = get_edt_body(&jsession).await;

    let a = get_ical_export_jid(&body).await;

    download_edt_ics_file(&jsession, &a.0, &a.1).await
}

#[get("/ics")]
async fn hello(req: HttpRequest) -> impl Responder {
    // returns 401 with a www challenge if no http basic auth header is given
    let Ok(creds) = authorization::Authorization::<Basic>::parse(&req) else {
        let challenge = www_authenticate::basic::Basic::new();
        return HttpResponse::Unauthorized()
            .insert_header(www_authenticate::WwwAuthenticate(challenge))
            .finish();
    };
    let (username, password) = (creds.as_ref().user_id(), creds.as_ref().password().unwrap());
    HttpResponse::Ok().body(download_ical(username, password).await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
