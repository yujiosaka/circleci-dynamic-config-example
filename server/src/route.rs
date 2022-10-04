use actix_web::{get, web::Query, HttpResponse, Responder};
use serde::Deserialize;
use shared;

#[derive(Clone, Deserialize)]
struct Params {
    left: f64,
    right: f64,
}

#[get("/add")]
async fn add((params,): (Query<Params>,)) -> impl Responder {
    HttpResponse::Ok().json(shared::add(params.left, params.right))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web::Bytes, App};

    #[actix_web::test]
    async fn test_add() {
        let app = test::init_service(App::new().service(add)).await;
        let req = test::TestRequest::get()
            .uri("/add?left=2&right=2")
            .to_request();
        let res = test::call_and_read_body(&app, req).await;
        assert_eq!(res, Bytes::from("4.0"));
    }
}
