pub async fn get_new_cas_execution_value() -> String {
    let body = reqwest::get("https://cas.uphf.fr/cas/login")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let doc = scraper::Html::parse_document(&body);

    let selector = scraper::Selector::parse(r#"input[name="execution"]"#).unwrap();

    let el = doc.select(&selector).next().unwrap().value();
    el.attr("value").unwrap().to_owned()
}

pub async fn get_cas_tgc_cookie(username: &str, password: &str, execution_value: &str) -> String {
    let client = config::get_reqwest_client();
    let form_params = [
        ("username", username),
        ("password", password),
        ("execution", execution_value),
        ("_eventId", "submit"),
    ];

    let response = client
        .post("https://cas.uphf.fr/cas/login")
        .form(&form_params)
        .send()
        .await
        .unwrap();
    let cookie = response.cookies().find(|cookie| cookie.name() == "TGC");
    cookie.unwrap().value().to_owned()
}
