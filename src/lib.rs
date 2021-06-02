mod auth;
mod me;
mod responses;
mod subreddit;
mod user;
mod utils;
mod message;

#[cfg(test)]
mod tests {
    use tokio;

    use crate::auth::{AnonymousAuthenticator, PasswordAuthenticator};
    use crate::me::{Me, FullName};
    use crate::responses::RedditType;
    use crate::responses::RedditType::{Comment, Link};
    use serde_json::Value;
    use std::str::FromStr;

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
    async fn user_saved() {
        dotenv::dotenv().ok();
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY").unwrap().as_str(),
            std::env::var("CLIENT_SECRET").unwrap().as_str(),
            std::env::var("REDDIT_USER").unwrap().as_str(),
            std::env::var("PASSWORD").unwrap().as_str(),
        );
        let me = Me::login(
            arc,
            "async_rawr test (by u/KingTuxWH)".to_string(),
        )
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
    #[tokio::test]
    async fn test_inbox() {
        dotenv::dotenv().ok();
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY").unwrap().as_str(),
            std::env::var("CLIENT_SECRET").unwrap().as_str(),
            std::env::var("REDDIT_USER").unwrap().as_str(),
            std::env::var("PASSWORD").unwrap().as_str(),
        );
        let me = Me::login(
            arc,
            "async_rawr test (by u/KingTuxWH)".to_string(),
        )
            .await
            .unwrap();
        let inbox = me.inbox();
        for x in inbox.get_messages(None, None).await.unwrap().data.children {
            match x.data {
                Comment(c) => {
                    println!("Comment {:?}", c.name);
                }
                RedditType::Account(a) => {}
                Link(l) => {}
                RedditType::Message(m) => {
                    println!("Message {:?}", m.name);
                }
                RedditType::Subreddit(s) => {}
                RedditType::Award => {}
            }
        }
    }
    #[tokio::test]
    async fn test_block() {
        dotenv::dotenv().ok();
        let arc = PasswordAuthenticator::new(
            std::env::var("CLIENT_KEY").unwrap().as_str(),
            std::env::var("CLIENT_SECRET").unwrap().as_str(),
            std::env::var("REDDIT_USER").unwrap().as_str(),
            std::env::var("PASSWORD").unwrap().as_str(),
        );
        let me = Me::login(
            arc,
            "async_rawr test (by u/KingTuxWH)".to_string(),
        )
            .await
            .unwrap();
        let inbox = me.inbox();
       inbox.block_author(FullName::from_str("t2_a3bjd54v").unwrap()).await;
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
