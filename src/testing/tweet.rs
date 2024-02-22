use actix_web::{get, http::header::ContentType, post, web::Json, HttpResponse};
use chrono::{DateTime, Utc};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

static TWEET_DATA: Lazy<Mutex<Tweet>> = Lazy::new(|| {
    Mutex::new(Tweet {
        id: None,
        created_at: None,
        message: None,
    })
});

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub message: Option<String>,
}

impl Tweet {
    pub fn new() -> Self {
        Self {
            id: Some(Uuid::new_v4().to_string()),
            created_at: Some(Utc::now()),
            message: None,
        }
    }
}

// Header Validation
#[derive(Debug, Deserialize, Serialize)]
pub struct TweetRequest {
    pub message: Option<String>,
}
// Header Controller
impl TweetRequest {
    pub fn to_tweet(&self) -> Option<Tweet> {
        match &self.message {
            Some(message) => {
                let mut var_return_data = Tweet::new();
                var_return_data.message = Some(message.to_string());
                TWEET_DATA.lock().unwrap().id = var_return_data.id.clone();
                TWEET_DATA.lock().unwrap().created_at = var_return_data.created_at.clone();
                TWEET_DATA.lock().unwrap().message = var_return_data.message.clone();
                return Some(var_return_data);
            }
            None => None,
        }
    }
}
// Routes
#[post("/tweets")]
pub async fn create(tweet_req: Json<TweetRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(ContentType::json())
        .json(tweet_req.to_tweet())
}

#[get("/tweets")]
pub async fn list() -> HttpResponse {
    let mut result = Tweet::new();
    // Locking the Variable and unwrap it from lock then clone the single memory.
    result.id = TWEET_DATA.lock().unwrap().id.clone();
    result.created_at = TWEET_DATA.lock().unwrap().created_at.clone();
    result.message = TWEET_DATA.lock().unwrap().message.clone();
    HttpResponse::Created()
        .content_type(ContentType::json())
        .json(result)
}

// #[get("/tweets/{id}")]
// pub async fn get() -> HttpResponse {
//     let found_tweet: Option<Tweet> = None;

//     match found_tweet {
//         Some(tweet) => HttpResponse::Ok()
//             .content_type(ContentType::json())
//             .json(tweet)
//         None => HttpResponse::NoContent()
//             .await
//             .unwrap(),
//     }
// }

// #[delete("/tweet/{id}")]
// pub async fn delete() -> HttpResponse {
//     HttpResponse::NoContent()
//         .await
//         .unwrap()
// }
