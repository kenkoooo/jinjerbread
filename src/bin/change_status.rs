extern crate jinjerbread;

use std::env;

use jinjerbread::model::{Check, Status};
use jinjerbread::operation;

fn main() {
    let args: Vec<String> = env::args().collect();
    let company_code = args[1].parse().unwrap();
    let email = &args[2];
    let password = &args[3];

    let result = operation::login(company_code, email, password).unwrap();
    let status = operation::get_status(&result.data);
    match status {
        Status::Working => println!(
            "{}",
            operation::check(&result.data, Check::Out).unwrap().message
        ),
        Status::CheckedOut => println!(
            "{}",
            operation::check(&result.data, Check::In).unwrap().message
        ),
        _ => {}
    }
}
