mod auth;
mod me;
mod responses;
mod subreddit;
mod user;
mod utils;

#[cfg(test)]
mod tests {
    use tokio;

    

    use crate::auth::{AnonymousAuthenticator};
    use crate::me::Me;
    use crate::responses::RedditType;
    

    #[tokio::test]
    async fn anon_subreddit_tests() {
        let me = Me::login(
            AnonymousAuthenticator::new(),
            "async_rawr test (by u/KingTuxWH)".to_string(),
        )
        .await
        .unwrap();
        let subreddit = me.subreddit("memes".to_string());
        let x = subreddit.about().await;
        let subreddit = x.unwrap();
        println!("{}", subreddit.data.title.unwrap());
    }

    #[tokio::test]
    async fn anon_user_tests() {
        let me = Me::login(
            AnonymousAuthenticator::new(),
            "async_rawr test (by u/KingTuxWH)".to_string(),
        )
        .await
        .unwrap();
        let user = me.user("KingTuxWH".to_string());
        let response = user.about().await.unwrap();
        println!("{}", response.data.name);
    }
}
