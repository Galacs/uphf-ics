use uphf_edt::*;

#[tokio::main]
async fn main() {
    let execution_value = dbg!(uphf_auth::get_new_cas_execution_value().await);

    let cookie = dbg!(
        uphf_auth::get_cas_tgc_cookie("remi.ait-younes", "Ilanderdu27!", &execution_value).await
    );

    let jsession = dbg!(uphf_edt::get_edt_jsession_id(&cookie).await);

    let body = get_edt_body(&jsession).await;

    let a = dbg!(get_ical_export_jid(&body).await);

    let ical = dbg!(download_edt_ics_file(&jsession, &a.0, &a.1).await);
}
