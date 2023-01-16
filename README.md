# wallet
Crypto portfolio and wallet

## Backend
1. install Rust by following the instructions [here](https://doc.rust-lang.org/stable/book/ch01-01-installation.html).
2. run the server by navigating to `/server` and executing `cargo run`

The server runs on port `8000`.

#### Endpoints
```bash
GET /tokens/{wallet-address}

GET /nfts/{wallet-address}

GET /transactions/{wallet-address}
```