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
