use openssl::hash::MessageDigest;
use openssl::pkey::{PKey};
use openssl::x509::{X509};
use std::net::SocketAddr;
use tracing::*;
use hudsucker::{
    async_trait::async_trait,
    certificate_authority::OpensslAuthority,
    hyper::{Body, Request, Response},
    *,
};




// for shutdown proxy
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

#[derive(Clone)]
struct LogHandler {}

#[async_trait]
impl HttpHandler for LogHandler {

    async fn handle_request(
        &mut self,
        _client_ctx: &HttpContext,
        request: Request<Body>,
    ) -> RequestOrResponse {                  
        println!("{:?}\n", request);
        println!("context: {:?}\n", _client_ctx);        
        RequestOrResponse::Request(request)
    }
    async fn handle_response(&mut self, _client_ctx: &HttpContext, response: Response<Body>) -> Response<Body> {
        println!("handle response:\n{:?}\n", response);
        response
    }
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // constant settings proxy
    let host_name: [u8; 4] = [127, 0, 0, 1];
    let bind_port: u16 = 8080;
    
    // get path private_key & rootCA_cert + convert in Byte
    let private_key_byte: &[u8] = include_bytes!("../../CA/CA-ROOT.key");
    let rootca_cert_byte: &[u8] = include_bytes!("../../CA/CA-ROOT.crt");
    
    // generate cert signed by ca 
    let private_key = PKey::private_key_from_pem(private_key_byte).expect("Failed to parse private_key");
    let ca_cert: X509 = X509::from_pem(rootca_cert_byte).expect("Failed to parse CA certificate");

    let ca = OpensslAuthority::new(private_key, ca_cert, MessageDigest::sha256(), 100_000);
    
    // configuration of proxy
    let proxy = ProxyBuilder::new()
        .with_addr(SocketAddr::from((host_name, bind_port)))
        .with_rustls_client()
        .with_ca(ca)
        .with_http_handler(LogHandler {})
        .build();
    println!("\nproxy is starting\n");

    if let Err(e) = proxy.start(shutdown_signal()).await {
        error!("{}", e);
    }
    println!("\nproxy shutdown\n");
}