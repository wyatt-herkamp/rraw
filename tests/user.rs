#[cfg(test)]
mod user_tests {
    fn init() {
        env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Trace)
            .try_init();
    }
    #[tokio::test]
    pub async fn test_search() -> anyhow::Result<()> {
        init();
        let client =
            Client::login(AnonymousAuthenticator::new(), "RRAW Test (by u/KingTuxWH)").await?;
        let users = client.search_users("King", None, None).await?;
        Ok(())
    }
}
