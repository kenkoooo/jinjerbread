extern crate jinjerbread;

use std::env;

use jinjerbread::model::Check;
use jinjerbread::operation;

fn main() {
    let args: Vec<String> = env::args().collect();
    let company_code = args[1].parse().unwrap();
    let email = &args[2];
    let password = &args[3];
    let check = match args[4].as_str() {
        "in" => Check::In,
        "out" => Check::Out,
        _ => unimplemented!(),
    };
    match operation::login(company_code, email, password)
        .and_then(|result| operation::check(&result, check))
    {
        Ok(c) => println!("{}", c.message),
        Err(err) => println!("{:?}", err),
    }
}
