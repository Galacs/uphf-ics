pub fn get_reqwest_client() -> reqwest::Client {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap()
}
