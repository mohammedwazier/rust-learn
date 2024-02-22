pub type Likes = Response<Like>;

pub struct Like {
    pub id: String,
    pub created_at: DateTime<Utc>,
}

impl Like {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }
}

#[get("/tweet/{id}/likes")]
pub async fn list(path: Path<(String,)>) -> HttpResponse {
    let likes = Likes { result: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

#[post("/tweet/{id}/likes")]
pub async fn plus_one(path: Path<(String,)>) -> HttpResponse {
    let like = Like::new();

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(like)
}

#[delete("/tweet/{id}/likes")]
pub async fn minus_one(path: Path<(String,)>) -> HttpResponse {
    // ToDo -> No Response
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
