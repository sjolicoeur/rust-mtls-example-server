# Sample project to experiment with mtls setup ( client authentication )

This is a sample project to have mutual-tls, client authentication, with an Actix Rust based webserver.

## build and run: 

- Generate certificates (look into `bin/create_certs.sh`) 
- `cargo run`

## Test it out
use the `.p12` file in the `ca/client` directory for browser testing. 

Or use the `.pem` file for curl:

```
curl -L  https://127.0.0.1:8443/ -k --cert ca/client/client.pem -v
```