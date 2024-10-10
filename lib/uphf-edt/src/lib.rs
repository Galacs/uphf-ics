use scraper::error::SelectorErrorKind;
use secrecy::{ExposeSecret, SecretBox, SecretString};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum EdtError {
    #[snafu(context(false), display("Http reqwest error: {source}"))]
    Http { source: reqwest::Error },
    #[snafu(context(false), display("Html Selector error: {source}"))]
    HtmlSelector { source: SelectorErrorKind<'static> },
    #[snafu(context(true), display("Html parse error: {msg}"))]
    HtmlParse { msg: String },
    #[snafu(context(true), display("Url parse error: {msg}"))]
    UrlParse { msg: String },
}

#[cfg_attr(feature = "instrument", tracing::instrument)]
pub async fn get_edt_jsession_id(tgc_cookie: SecretBox<str>) -> Result<SecretBox<str>, EdtError> {
    let client = config::get_reqwest_client();
    let response = client
        .get("https://cas.uphf.fr/cas/login?service=https://vtmob.uphf.fr/esup-vtclient-up4/stylesheets/desktop/welcome.xhtml")
        .header("Cookie", format!("TGC={}", tgc_cookie.expose_secret()))
        .send()
        .await?;
    // TODO: maybe try to get JSESSIONID from the header in a previous redirect?
    let url = response.url().to_string();
    Ok(SecretString::from(
        url.split("=")
            .last()
            .context(UrlParse {
                msg: "no JSESSIONID in redirect url",
            })?
            .to_owned(),
    ))
}

#[cfg_attr(feature = "instrument", tracing::instrument)]
pub async fn get_edt_body(jsession_id: SecretBox<str>) -> Result<String, EdtError> {
    let client = config::get_reqwest_client();
    let response = client
        .get("https://vtmob.uphf.fr/esup-vtclient-up4/stylesheets/desktop/welcome.xhtml")
        .header(
            "Cookie",
            format!("JSESSIONID={}", jsession_id.expose_secret()),
        )
        .send()
        .await?;
    Ok(response.text().await?)
}

#[cfg_attr(feature = "instrument", tracing::instrument)]
pub async fn get_ical_export_jid(body: &str) -> Result<(String, String), EdtError> {
    let doc = scraper::Html::parse_document(body);

    let selector = scraper::Selector::parse(r#"a[title="Export iCal"]"#)?;

    let el = doc
        .select(&selector)
        .next()
        .context(HtmlParse {
            msg: "no export ical link",
        })?
        .value();
    let onclick = el
        .attr("onclick")
        .context(HtmlParse {
            msg: "no onclik attr on a",
        })?
        .to_owned();

    let parse_err = HtmlParse {
        msg: "onclick extraction error",
    };

    let mut inside = onclick
        .split("(")
        .last()
        .context(parse_err)?
        .split(")")
        .next()
        .context(parse_err)?
        .split(",");
    let a = inside.next().context(parse_err)?.replace("'", "");
    let b = inside.next().context(parse_err)?.replace("'", "");
    Ok((a, b))
}

#[cfg_attr(feature = "instrument", tracing::instrument)]
pub async fn download_edt_ics_file(
    jsession_id: SecretBox<str>,
    form: &str,
    idcl: &str,
) -> Result<String, EdtError> {
    let form_params = [
        ("org.apache.myfaces.trinidad.faces.FORM", form),
        (&format!("{}:_idcl", form), idcl),
        ("javax.faces.ViewState", "!1"),
    ];

    let client = config::get_reqwest_client();
    let response = client
        .post("https://vtmob.uphf.fr/esup-vtclient-up4/stylesheets/desktop/welcome.xhtml")
        .form(&form_params)
        .header(
            "Cookie",
            format!("JSESSIONID={}", jsession_id.expose_secret()),
        )
        .send()
        .await?;

    Ok(response.text().await?)
}
