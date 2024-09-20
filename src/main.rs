#[tokio::main]
async fn main() {
    dbg!(uphf_auth::get_new_cas_execution_value().await);
}
