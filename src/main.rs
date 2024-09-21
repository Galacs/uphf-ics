async fn get_edt_body(jsession_id: &str) -> String {
    let client = config::get_reqwest_client();
    let response = client
        .get("https://vtmob.uphf.fr/esup-vtclient-up4/stylesheets/desktop/welcome.xhtml")
        .header("Cookie", format!("JSESSIONID={}", jsession_id))
        .send()
        .await
        .unwrap();
    response.text().await.unwrap()
}

async fn get_ical_export_jid(body: &str) -> (String, String) {
    let doc = scraper::Html::parse_document(body);

    let selector = scraper::Selector::parse(r#"a[title="Export iCal"]"#).unwrap();

    let el = doc.select(&selector).next().unwrap().value();
    let onclick = el.attr("onclick").unwrap().to_owned();
    let mut inside = onclick
        .split("(")
        .last()
        .unwrap()
        .split(")")
        .next()
        .unwrap()
        .split(",");
    let a = inside.next().unwrap().replace("'", "");
    let b = inside.next().unwrap().replace("'", "");
    (a, b)
}

async fn download_edt_ics_file(jsession_id: &str, form: &str, idcl: &str) -> String {
    let form_params = [
        ("org.apache.myfaces.trinidad.faces.FORM", form),
        (&format!("{}:_idcl", form), idcl),
        ("javax.faces.ViewState", "!1"),
    ];

    let client = config::get_reqwest_client();
    let response = client
        .post("https://vtmob.uphf.fr/esup-vtclient-up4/stylesheets/desktop/welcome.xhtml")
        .form(&form_params)
        .header("Cookie", format!("JSESSIONID={}", jsession_id))
        .send()
        .await
        .unwrap();

    response.text().await.unwrap()
}

#[tokio::main]
async fn main() {
    let execution_value = dbg!(uphf_auth::get_new_cas_execution_value().await);

    let cookie = dbg!(uphf_auth::get_cas_tgc_cookie("", "", &execution_value).await);

    let jsession = dbg!(uphf_edt::get_edt_jsession_id(&cookie).await);

    let body = get_edt_body(&jsession).await;

    let a = dbg!(get_ical_export_jid(&body).await);

    let ical = dbg!(download_edt_ics_file(&jsession, &a.0, &a.1).await);
}
