#!/usr/bin/bash

mkdir -p ca/{client,server}

openssl genrsa -aes256 -out ca/ca.key 4096 chmod 400 ca/ca.key

openssl genrsa -aes256 -out ca/ca.key 4096 chmod 400 ca/ca.key
openssl req -new -x509 -sha256 -days 730 -key ca/ca.key -out ca/ca.crt
chmod 444 ca/ca.crt

openssl genrsa -out ca/server/client-ssl.key 2048
chmod 400 ca/server/client-ssl.key
openssl req -new -key ca/server/client-ssl.key \
        -sha256 -out server/client-ssl.csr

openssl x509 -req -days 365 -sha256 -in ca/server/server-ssl.csr \
        -CA ca/ca.crt -CAkey ca/ca.key -set_serial 1 -out ca/server/server-ssl.crt
openssl req -new -key ca/server/client-ssl.key -sha256 \
        -out ca/server/client-ssl.csr
openssl x509 -req -days 365 -sha256 -in ca/server/server-ssl.csr \
        -CA ca/ca.crt -CAkey ca/ca.key -set_serial 1 -out ca/server/server-ssl.crt
chmod 444 ca/server/server-ssl.crt

# verify
# openssl x509 -noout -text -in ca/server/server-ssl.crt

openssl genrsa -out ca/client/client.key 2048
openssl req -new -key ca/client/client.key -out ca/client/client.csr
openssl x509 -req -days 365 -sha256 -in ca/client/client.csr \
        -CA ca/ca.crt -CAkey ca/ca.key -set_serial 2 -out ca/client/client.crt

# generate pem file to use with curl
cat ca/client/client.crt ca/client/client.key > ca/client/client.pem
# generate cert file to use with browser -- client.p12
openssl pkcs12 -export -out client.p12 -in client.pem -inkey client.pem

# curl command
curl -L  https://127.0.0.1:8443/ -k --cert ca/client/client.p12 -v