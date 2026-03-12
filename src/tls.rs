//! TLS bootstrap primitives for MVP stage.
//! For production, certificates should come from ACME/PKI and the server should be
//! started with HTTPS listener. DTLS is handled by WebRTC stack (future mediasoup integration).

use rustls::ServerConfig;
use std::sync::Arc;

pub fn build_placeholder_tls_config() -> Arc<ServerConfig> {
    let provider = rustls::crypto::ring::default_provider();
    let config = rustls::ServerConfig::builder_with_provider(provider.into())
        .with_protocol_versions(&[&rustls::version::TLS13])
        .expect("TLS 1.3 should be available")
        .with_no_client_auth()
        .with_cert_resolver(Arc::new(rustls::server::ResolvesServerCertUsingSni::new()));
    Arc::new(config)
}
