use reqwest::{Certificate, Client, Identity, Response};
use serde_json::Value;
use std::error::Error;
use std::fs;

pub async fn send_authenticated_request(
    request_type: &str,
    endpoint: &str,
    content: &Value,
) -> Result<Response, Box<dyn Error>> {
    // Load client certificate and private key
    let cert = fs::read("./src/config/client-crt.pem")?;
    let key = fs::read("./src/config/client-key.pem")?;

    let mut cert_and_key = cert.clone();
    cert_and_key.extend_from_slice(&key);
    let client_identity = Identity::from_pem(&cert_and_key)?;

    // Load CA certificate
    let ca_cert = fs::read("./src/config/ca-crt.pem")?;
    let ca_certificate = Certificate::from_pem(&ca_cert)?;

    // Build the client with Rustls TLS backend
    let client = Client::builder()
        .identity(client_identity)
        .add_root_certificate(ca_certificate)
        .use_rustls_tls() // Use Rustls as the TLS backend
        .build()?;

    let response: Response;

    match request_type {
        "get" => {
            response = client.get(endpoint).send().await?;
        }
        "post" => {
            response = client.post(endpoint).json(content).send().await?;
        }
        _ => return Err(format!("Unsupported request type: {}", request_type).into()),
    }
    
    Ok(response)
}
