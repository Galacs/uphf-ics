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

fn get_reqwest_client() -> reqwest::Client {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap()
}

pub async fn get_cas_tgc_cookie(username: &str, password: &str, execution_value: &str) -> String {
    let client = get_reqwest_client();
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

pub async fn get_edt_jsession_id(tgc_cookie: &str) -> String {
    let client = get_reqwest_client();
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
