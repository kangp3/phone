- Generate v3 certs by comparing v3 cert gen against Asterisk script
  - https://www.golinuxcloud.com/openssl-create-certificate-chain-linux/
  - https://github.com/asterisk/asterisk/blob/master/contrib/scripts/ast_tls_cert
  - `openssl x509 -text -noout -in frandline.pem`
- Use and commit openssl config files

cargo run --bin tlser
