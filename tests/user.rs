#[cfg(test)]
mod user_tests {
    use log::LevelFilter;
    use rraw::auth::AnonymousAuthenticator;
    use rraw::Client;
    pub static TEST_USERS: [&str; 3] = ["KingTuxWH", "TheSmartKing", "Princeflower13"];

    fn init() {
        if let Err(error) = env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Debug)
            .try_init()
        {
            println!("Logger Failed to Init Error: {}", error);
        }
    }
    #[tokio::test]
    pub async fn test_search() -> anyhow::Result<()> {
        init();
        let client =
            Client::login(AnonymousAuthenticator::new(), "RRAW Test (by u/KingTuxWH)").await?;
        let users = client.search_users("King", None, None).await?;
        assert!(users.data.children.len() > 1);
        Ok(())
    }
    #[tokio::test]
    pub async fn test_get_user() -> anyhow::Result<()> {
        init();
        let client =
            Client::login(AnonymousAuthenticator::new(), "RRAW Test (by u/KingTuxWH)").await?;
        for username in TEST_USERS {
            let user = client.user(username).await;
            assert!(
                user.is_ok(),
                "{}/about could not be loaded correctly",
                username
            );
            let user = user.unwrap();
            assert!(
                user.submissions(None).await.is_ok(),
                "{}/submissions could not be loaded correctly",
                username
            );
            assert!(
                user.comments(None).await.is_ok(),
                "{}/comments could not be loaded correctly",
                username
            );
        }

        Ok(())
    }
}
