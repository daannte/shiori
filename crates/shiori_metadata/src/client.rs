use once_cell::sync::Lazy;
use reqwest::Client;

pub(crate) static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .user_agent("shiori/0.1")
        .pool_max_idle_per_host(10)
        .build()
        .expect("Failed to build client")
});
