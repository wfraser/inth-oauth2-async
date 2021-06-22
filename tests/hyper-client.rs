use hyper_tls::HttpsConnector;
use inth_oauth2_async::{
    Client,
    error::{OAuth2Error, OAuth2ErrorCode},
    client::ClientError,
    provider,
};

// Mostly this test is just to verify that the client abstraction works with a typical Hyper client
// instantiation.
#[tokio::test]
async fn test_hyper_client() {
    let tls = HttpsConnector::new();
    let hyper = hyper::client::Client::builder().build::<_, hyper::Body>(tls);

    let client = Client::new(
        provider::google::Web,
        "foo".to_owned(),
        "bar".to_owned(),
        Some("https://example.com/squeedleedee".to_owned()),
    );

    let result = client.request_token(&hyper, "spoopadoop")
        .await;

    eprintln!("result: {:?}", result);
    assert!(matches!(
        result,
        Err(ClientError::OAuth2(OAuth2Error { code: OAuth2ErrorCode::InvalidClient, .. }))
    ));
}
