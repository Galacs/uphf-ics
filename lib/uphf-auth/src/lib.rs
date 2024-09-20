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
