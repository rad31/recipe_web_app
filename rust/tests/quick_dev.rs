use serde_json::json;

#[tokio::test]
async fn quick_dev() -> () {
    let hc  = httpc_test::new_client("http://localhost:8080").unwrap();
    let login_request_success = hc.do_post(
        "/api/login",
        json!({
            "username": "ronny",
            "password": "password",
        })
    );
    
    login_request_success.await.unwrap().print().await.unwrap();
}