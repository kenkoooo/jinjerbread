# JinjerBread

A library to operate [Jinjer](https://kintai.jinjer.biz/) by Rust.

# Requirement
- Rust (>=2018)
- Cargo

# Example Application
## CLI tool to sign in/out [./src/bin/change_status.rs](./src/bin/change_status.rs)
```bash
cargo run --bin change_status [company_code] [email] [password]

# Example
cargo run --bin change_status 1234 kenkou@company.example.com this_is_password
```
