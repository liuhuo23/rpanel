use actix_web::{App, HttpRequest, HttpResponse, get, http::header::ContentType, test, web};

// Define AppState struct for testing
#[derive(Debug, Clone, serde::Serialize)]
pub struct AppState {
    pub message: String,
}

#[get("/")]
pub async fn index(req: HttpRequest) -> HttpResponse {
    if let Some(state) = req.app_data::<web::Data<AppState>>() {
        HttpResponse::Ok().json(state.get_ref())
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[actix_web::test]
async fn test_index_ok() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
