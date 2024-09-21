pub async fn get_edt_jsession_id(tgc_cookie: &str) -> String {
    let client = config::get_reqwest_client();
    let response = client
        .get("https://cas.uphf.fr/cas/login?service=https://vtmob.uphf.fr/esup-vtclient-up4/stylesheets/desktop/welcome.xhtml")
        .header("Cookie", format!("TGC={}", tgc_cookie))
        .send()
        .await
        .unwrap();
    // TODO: maybe try to get JSESSIONID from the header in a previous redirect?
    let url = response.url().to_string();
    url.split("=").last().unwrap().to_owned()
}

pub async fn get_edt_body(jsession_id: &str) -> String {
    let client = config::get_reqwest_client();
    let response = client
        .get("https://vtmob.uphf.fr/esup-vtclient-up4/stylesheets/desktop/welcome.xhtml")
        .header("Cookie", format!("JSESSIONID={}", jsession_id))
        .send()
        .await
        .unwrap();
    response.text().await.unwrap()
}

pub async fn get_ical_export_jid(body: &str) -> (String, String) {
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

pub async fn download_edt_ics_file(jsession_id: &str, form: &str, idcl: &str) -> String {
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
