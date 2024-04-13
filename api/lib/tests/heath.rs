use actix_web::App;
use api_lib::health::service;

#[actix_rt::test]
async fn health_check_work() {
    let app = App::new().configure(service);
    let mut app = actix_web::test::init_service(app).await;
    //construct a request
    let req = actix_web::test::TestRequest::get()
        .uri("/health")
        .to_request();
    //do the request and return the response
    let res = actix_web::test::call_service(&mut app, req).await;
    assert!(res.status().is_success());
    assert_eq!(res.status(), actix_web::http::StatusCode::OK);
    let version = res.headers().get("version").and_then(|v| v.to_str().ok());
    assert_eq!(version, Some(api_lib::health::API_VERSION));
}
