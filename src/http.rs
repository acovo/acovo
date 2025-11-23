#[cfg(feature = "http")]
mod tests {
    use anyhow::Result as AnyResult;
    use reqwest_middleware::ClientBuilder;
    use reqwest_proxy_pool::{ProxyPoolConfig, ProxyPoolMiddleware, ProxySelectionStrategy};
    use tokio::runtime::Runtime;

    use super::*;

    #[test]
    fn test_reqwest_proxy_pool() -> AnyResult<()> {
        //env_logger::init();

        println!("Initializing proxy pool...");

        let config = ProxyPoolConfig::builder()
            // free socks5 proxy urls, format like `Free-Proxy`
            .sources(vec!["https://raw.githubusercontent.com/dpangestuw/Free-Proxy/main/socks5_proxies.txt"])
            //.health_check_timeout(Duration::from_secs(10))
            //.health_check_url("https://www.google.com")
            .retry_count(2)
            .selection_strategy(ProxySelectionStrategy::FastestResponse)
            // rate limit for each proxy, lower performance but avoid banned
            .max_requests_per_second(3.0)
            .build();

        let rt = Runtime::new().unwrap();
        let ret: AnyResult<()> = rt.block_on(async move {
            let proxy_pool = ProxyPoolMiddleware::new(config).await?;

            let client = ClientBuilder::new(reqwest::Client::new())
                .with(proxy_pool)
                .build();

            println!("Sending request...");
            let response: reqwest::Response = client.get("https://httpbin.org/ip").send().await?;

            println!("Status: {}", response.status());
            println!("Response: {}", response.text().await.unwrap());
            Ok(())
        });
        match ret {
            Ok(()) => {}
            Err(e) => {
                println!("onError {}", e);
            }
        }

        Ok(())
    }
}
