# JinjerBread

A library to operate [Jinjer](https://kintai.jinjer.biz/) by Rust.

# Requirement
- Rust (>=2018)
- Cargo

# Example Applications
## CLI tool to sign in/out [./example/change_status.rs](./example/change_status.rs)

### Build 
By the following command, an executable `change_status` will be generated in `./target/debug/`.
```bash
cargo build
```

### Run
You need to fill the following enviroment variables to use this tool.
- `JINJER_CODE` Your company code
- `JINJER_EMAIL` Your company e-mail address
- `JINJER_PASSWORD` Your password

```bash
./change_status

# Example
env JINJER_CODE=12345 JINJER_EMAIL=kenkoooo@example.com JINJER_PASSWORD=this_is_password ./change_status
```
