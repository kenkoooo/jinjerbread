extern crate jinjerbread;

use std::env;

use jinjerbread::model::{Check, Status};
use jinjerbread::operation;

fn main() {
    let company_code = env::var("JINJER_CODE")
        .expect("Enviroment variable 'JINJER_CODE' is not set!")
        .parse::<usize>()
        .expect("Enviroment variable 'JINJER_CODE' is not a positive integer!");
    let email = env::var("JINJER_EMAIL").expect("Enviroment variable 'JINJER_EMAIL' is not set!");
    let password =
        env::var("JINJER_PASSWORD").expect("Enviroment variable 'JINJER_PASSWORD' is not set!");

    let result = operation::login(company_code, &email, &password).unwrap();
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
