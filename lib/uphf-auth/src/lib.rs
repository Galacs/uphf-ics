use scraper::error::SelectorErrorKind;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum AuthError {
    #[snafu(context(false), display("Http reqwest error: {source}"))]
    Http { source: reqwest::Error },
    #[snafu(context(false), display("Html Selector error: {source}"))]
    HtmlSelector { source: SelectorErrorKind<'static> },
    #[snafu(context(true), display("Html parse error: {msg}"))]
    HtmlParse { msg: String },
    #[snafu(context(true), display("Cookie parse error: {msg}"))]
    CookieParse { msg: String },
}

#[cfg_attr(feature = "instrument", tracing::instrument)]
pub async fn get_new_cas_execution_value() -> Result<String, AuthError> {
    let response = reqwest::get("https://cas.uphf.fr/cas/login").await?;
    let response = response.error_for_status()?;
    let body = response.text().await?;
    let doc = scraper::Html::parse_document(&body);

    let selector = scraper::Selector::parse(r#"input[name="execution"]"#)?;

    let el = doc
        .select(&selector)
        .next()
        .context(HtmlParse {
            msg: "no execution input element",
        })?
        .value();
    Ok(el
        .attr("value")
        .context(HtmlParse {
            msg: "no value attr inside input el",
        })?
        .to_owned())
}

#[cfg_attr(feature = "instrument", tracing::instrument)]
pub async fn get_cas_tgc_cookie(
    username: &str,
    password: &str,
    execution_value: &str,
) -> Result<String, AuthError> {
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
        .await?;
    let response = response.error_for_status()?;
    let cookie = response.cookies().find(|cookie| cookie.name() == "TGC");
    Ok(cookie
        .context(CookieParse {
            msg: "no TGC cookie",
        })?
        .value()
        .to_owned())
}
