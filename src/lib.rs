pub mod auth;
pub mod me;
pub mod message;
pub mod responses;
pub mod subreddit;
pub mod user;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde_json::Value;
    use tokio;

    use crate::auth::{AnonymousAuthenticator, PasswordAuthenticator};
    use crate::me::{FullName, Me};
    use crate::responses::RedditType;
    use crate::responses::RedditType::{Comment, Link};
    use crate::utils::options::FriendType;

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

    #[ignore]
    #[tokio::test]
    async fn user_saved() {
        dotenv::dotenv().ok();
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY").unwrap().as_str(),
            std::env::var("CLIENT_SECRET").unwrap().as_str(),
            std::env::var("REDDIT_USER").unwrap().as_str(),
            std::env::var("PASSWORD").unwrap().as_str(),
        );
        let me = Me::login(arc, "async_rawr test (by u/KingTuxWH)".to_string())
            .await
            .unwrap();
        let user = me.user("KingTuxWH".to_string());
        let x = user.saved(None).await.unwrap();
        for x in x.data.children {
            match x.data {
                Comment(comment) => {
                    println!("Comment {:?}", comment.body);
                }
                Link(link) => {
                    println!("Link {:?}", link.name);
                }
                _ => {}
            }
        }
    }

    #[ignore]
    #[tokio::test]
    async fn test_inbox() {
        dotenv::dotenv().ok();
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY").unwrap().as_str(),
            std::env::var("CLIENT_SECRET").unwrap().as_str(),
            std::env::var("REDDIT_USER").unwrap().as_str(),
            std::env::var("PASSWORD").unwrap().as_str(),
        );
        let me = Me::login(arc, "async_rawr test (by u/KingTuxWH)".to_string())
            .await
            .unwrap();
        let inbox = me.inbox();
        for x in inbox.get_messages(None, None).await.unwrap().data.children {
            match x.data {
                Comment(c) => {
                    println!("Comment {:?}", c.name);
                }
                RedditType::Account(_a) => {}
                Link(_l) => {}
                RedditType::Message(m) => {
                    println!("Message {:?}", m.name);
                }
                RedditType::Subreddit(_s) => {}
                RedditType::Award => {}
            }
        }
    }

    #[ignore]
    #[tokio::test]
    async fn hidden_sub() {
        dotenv::dotenv().ok();
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY").unwrap().as_str(),
            std::env::var("CLIENT_SECRET").unwrap().as_str(),
            std::env::var("REDDIT_USER").unwrap().as_str(),
            std::env::var("PASSWORD").unwrap().as_str(),
        );
        let me = Me::login(arc, "async_rawr test (by u/KingTuxWH)".to_string())
            .await
            .unwrap();
        let response = me.subreddit("RedditNobility".to_string()).about().await.unwrap();
    }

    #[ignore]
    #[tokio::test]
    async fn friend() {
        dotenv::dotenv().ok();
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY").unwrap().as_str(),
            std::env::var("CLIENT_SECRET").unwrap().as_str(),
            std::env::var("REDDIT_USER").unwrap().as_str(),
            std::env::var("PASSWORD").unwrap().as_str(),
        );
        let me = Me::login(arc, "async_rawr test (by u/KingTuxWH)".to_string())
            .await
            .unwrap();
        let response = me.subreddit("RedditNobility".to_string()).add_friend("LordPenguin42".to_string(), FriendType::Contributor).await.unwrap();
    }

    #[ignore]
    #[tokio::test]
    async fn test_send() {
        dotenv::dotenv().ok();
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY").unwrap().as_str(),
            std::env::var("CLIENT_SECRET").unwrap().as_str(),
            std::env::var("REDDIT_USER").unwrap().as_str(),
            std::env::var("PASSWORD").unwrap().as_str(),
        );
        let me = Me::login(arc, "async_rawr test (by u/KingTuxWH)".to_string())
            .await
            .unwrap();
        let inbox = me.inbox();
        let result = inbox.compose("LordPenguin42".to_string(),
                                   "Test from Async Rawr".to_string(),
                                   "I don’t want to talk to you no more, you empty-headed animal-food-trough wiper. I fart in your general direction. Your mother was a hamster, and your father smelt of elderberries.".to_string(),
                                   Some("new_rawr".to_string())).await;
        my_loop(result.unwrap().as_object().unwrap());
    }

    #[ignore]
    #[tokio::test]
    async fn test_block() {
        dotenv::dotenv().ok();
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY").unwrap().as_str(),
            std::env::var("CLIENT_SECRET").unwrap().as_str(),
            std::env::var("REDDIT_USER").unwrap().as_str(),
            std::env::var("PASSWORD").unwrap().as_str(),
        );
        let me = Me::login(arc, "async_rawr test (by u/KingTuxWH)".to_string())
            .await
            .unwrap();
        let inbox = me.inbox();
        inbox
            .block_author(FullName::from_str("t2_a3bjd54v").unwrap())
            .await;
    }

    #[tokio::test]
    async fn anon_user_tests() {
        let me = Me::login(
            AnonymousAuthenticator::new(),
            "async_rawr test (by u/KingTuxWH)".to_string(),
        )
            .await
            .unwrap();
        let user = me.user("HoodwinkingGnome".to_string());
        let result = user.about().await;
        if let Err(error) = result {
            println!("{}", error);
            return;
        }
        let response = result.unwrap();
        let submissions = user.submissions(None).await.unwrap();
        let comments = user.comments(None).await.unwrap();
        println!("{}", response.data.name);
    }

    fn my_loop(map: &serde_json::Map<String, Value>) {
        for x in map {
            if x.1.is_object() {
                my_loop(x.1.as_object().unwrap());
            } else if x.1.is_boolean() {
                println!("pub {}: Option<bool>,", x.0)
            } else if x.1.is_null() {
                println!("pub {}: Option<Value>,", x.0)
            } else if x.1.is_f64() {
                println!("pub {}: Option<f64>,", x.0)
            } else if x.1.is_i64() {
                println!("pub {}: Option<i64>,", x.0)
            } else if x.1.is_u64() {
                println!("pub {}: Option<u64>,", x.0)
            } else if x.1.is_string() {
                println!("pub {}: Option<String>,", x.0)
            } else if x.1.is_number() {
                println!("pub {}: Option<i64>,", x.0)
            }
        }
    }
}
