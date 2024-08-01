#!/bin/bash

# Check if server CN is provided as an argument
if [ -z "$1" ]; then
  echo "Usage: $0 <server_cn>"
  exit 1
fi

SERVER_CN=$1

# Generate a self-signed root CA private 
openssl req -x509 -sha256 -nodes -subj "/C=US/CN=MyCA" -days 1825 -newkey rsa:2048 -keyout rootCA.key -out rootCA.crt ######

# Generate the server private key and CSR
openssl req -newkey rsa:2048 -nodes -subj "/C=US/CN=$SERVER_CN" -keyout server-key.pem -out server-key.csr ###########

# Generate the client private key and CSR
openssl req -newkey rsa:2048 -nodes -subj "/C=US/CN=client" -keyout client-key.pem -out client-key.csr ###########

# Create file localhost.ext with the following content:
cat << EOF >> localhost.ext
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
subjectAltName = @alt_names
[alt_names]
DNS.1 = server
IP.1 = $SERVER_CN
EOF

# Sign the client CSR (`cert.pem`) with the root CA certificate and private key
# => this overwrites `cert.pem` because it gets signed
openssl x509 -req -CA rootCA.crt -CAkey rootCA.key -in client-key.csr -out client-crt.pem -days 365 -CAcreateserial -extfile localhost.ext

# Sign the server CSR
openssl x509 -req -CA rootCA.crt -CAkey rootCA.key -in server-key.csr -out server-crt.pem -days 365 -CAcreateserial -extfile localhost.ext

echo "Certificates generated successfully."
