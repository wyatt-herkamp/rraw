mod me;
mod auth;
mod responses;
mod subreddit;
mod user;
mod utils;

#[cfg(test)]
mod tests {
    use crate::me::Me;
    use crate::auth::AnonymousAuthenticator;
    use tokio;

    #[tokio::test]
    async fn anon_subreddit_tests() {
        let me = Me::login(AnonymousAuthenticator::new(), "async_rawr test (by u/KingTuxWH)".to_string()).await.unwrap();
        let subreddit = me.subreddit("memes".to_string());
        let x = subreddit.about().await;
        let subreddit1 = x.unwrap();
        println!("{}", subreddit1.data.title.unwrap());
    }

    #[tokio::test]
    async fn anon_user_tests() {
        let me = Me::login(AnonymousAuthenticator::new(), "async_rawr test (by u/KingTuxWH)".to_string()).await.unwrap();
        let user = me.user("KingTuxWH".to_string());
        let about = user.about().await.unwrap();
        println!("{}", about.data.name);
    }
}
