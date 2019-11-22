use std::io;

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslVerifyMode, SslSessionCacheMode};
use openssl::x509::store::{X509StoreBuilder};
use openssl::x509::{X509};
use openssl::ssl::{SslVersion};
use std::fs;

/// simple handle
fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome! From rust! with certs?\n"))
}

fn drats(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hmmm... Where you supposed to get here?\n"))
}


fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let sys_server = actix_rt::System::new("tls-example");

    // load ssl keys
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    let mut builder = SslAcceptor::mozilla_modern(SslMethod::tls())?;
    builder.set_private_key_file("ca/server/client-ssl.key", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("ca/server/client-ssl.crt")?;

    let ca_cert = fs::read_to_string("ca/ca.crt")?.into_bytes();
    let client_ca_cert = X509::from_pem(&ca_cert)?;
    let mut x509_client_store_builder = X509StoreBuilder::new()?;
    x509_client_store_builder.add_cert(client_ca_cert)?;
    let client_cert_store = x509_client_store_builder.build();
    builder.set_verify_cert_store(client_cert_store)?;

    // set options to make sure to validate the peer aka mtls
    let mut verify_mode = SslVerifyMode::empty();
    verify_mode.set(SslVerifyMode::PEER, true);
    verify_mode.set(SslVerifyMode::FAIL_IF_NO_PEER_CERT, true);
    builder.set_verify(verify_mode);

    // may not need to set it to off:
    // https://www.openssl.org/docs/man1.0.2/man3/SSL_CTX_set_session_cache_mode.html
    // https://vincent.bernat.ch/en/blog/2011-ssl-session-reuse-rfc5077
    builder.set_session_cache_mode(SslSessionCacheMode::OFF);
    let min_ssl_version_3 = Some(SslVersion::SSL3);
    builder.set_min_proto_version(min_ssl_version_3)?;

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler, handle all methods
            .service(web::resource("/index.html").to(index))
            .service(web::resource("/secret").to(drats))
            // with path parameters
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "/index.html")
                    .finish()
            })))
    })
        .bind_ssl("127.0.0.1:8443", builder)?
        .start();

    println!("Started http server: 127.0.0.1:8443");
    sys_server.run()
}