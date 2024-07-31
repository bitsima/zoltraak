#!/bin/bash

# Check if server CN is provided as an argument
if [ -z "$1" ]; then
  echo "Usage: $0 <server_cn>"
  exit 1
fi

SERVER_CN=$1

# Generate the CA private key
openssl genpkey -algorithm RSA -out ca-key.pem -pkeyopt rsa_keygen_bits:2048

# Generate the CA certificate
openssl req -x509 -new -nodes -key ca-key.pem -sha256 -days 1024 -out ca-crt.pem -subj "/CN=MyCA"

# Generate the server private key
openssl genpkey -algorithm RSA -out server-key.pem -pkeyopt rsa_keygen_bits:2048

# Generate the server CSR
openssl req -new -key server-key.pem -out server-csr.pem -subj "/CN=$SERVER_CN"

# Generate the server certificate
openssl x509 -req -in server-csr.pem -CA ca-crt.pem -CAkey ca-key.pem -CAcreateserial -out server-crt.pem -days 1024 -sha256

# Generate the client private key
openssl genpkey -algorithm RSA -out client-key.pem -pkeyopt rsa_keygen_bits:2048

# Generate the client CSR
openssl req -new -key client-key.pem -out client-csr.pem -subj "/CN=Client"

# Generate the client certificate
openssl x509 -req -in client-csr.pem -CA ca-crt.pem -CAkey ca-key.pem -CAcreateserial -out client-crt.pem -days 1024 -sha256

echo "Certificates generated successfully."
