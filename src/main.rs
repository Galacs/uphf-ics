#[tokio::main]
async fn main() {
    let execution_value = dbg!(uphf_auth::get_new_cas_execution_value().await);

    let cookie = dbg!(uphf_auth::get_cas_tgc_cookie("", "", &execution_value).await);
}
