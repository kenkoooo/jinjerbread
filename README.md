# JinjerBread

A library to operate [Jinjer](https://kintai.jinjer.biz/) by Rust.

# Requirement
- Rust (>=2018)
- Cargo

# Example Application
## CLI tool to sign in/out [./src/bin/check.rs](./src/bin/check.rs)
```bash
cargo run --bin check [company_code] [email] [password] in # sign in
cargo run --bin check [company_code] [email] [password] out # sign out 

# Example
cargo run --bin check 1234 kenkou@company.example.com this_is_password out
```
