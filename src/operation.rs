use crate::model::{Check, CheckResult, LoginData, LoginInfo, LoginResult, Status, TimeCard};
use chrono::prelude::*;
use chrono_tz::Asia::Tokyo;
use reqwest::Response;
use serde_json::Value;

const ROOT_URL: &str = "https://kintai.jinjer.biz/v1";

pub fn login(
    company_code: usize,
    email: &str,
    password: &str,
) -> Result<LoginResult, reqwest::Error> {
    let info = LoginInfo {
        company_code: company_code,
        email: email.to_owned(),
        password: password.to_owned(),
        remember_me: true,
    };

    let url = ROOT_URL.to_owned() + "/sign_in";
    let client = reqwest::Client::new();
    client
        .post(&url)
        .json(&info)
        .send()
        .and_then(|mut result| result.json::<LoginResult>())
}

pub fn check(data: &LoginData, check_kind: Check) -> Result<CheckResult, reqwest::Error> {
    let url = format!("{}/dashboard/shops/{}/time_cards", ROOT_URL, data.shop_id);
    let time_card = TimeCard::new(check_kind);
    reqwest::Client::new()
        .post(&url)
        .header("Api-Token", data.token.clone())
        .json(&time_card)
        .send()
        .and_then(|mut result| result.json::<CheckResult>())
}

fn parse_info(mut value: Value) -> Option<Status> {
    value
        .get_mut("data")
        .map(|d: &mut Value| d.take())
        .and_then(|mut v: Value| v.get_mut(0).map(|d: &mut Value| d.take()))
        .and_then(|mut v: Value| v.get_mut("users").map(|d: &mut Value| d.take()))
        .and_then(|mut v: Value| v.get_mut(0).map(|d: &mut Value| d.take()))
        .and_then(|mut v: Value| v.get_mut("user_info").map(|d: &mut Value| d.take()))
        .and_then(|mut v| v.get_mut("last_status_time_card").map(|d: &mut _| d.take()))
        .and_then(|v| match v {
            serde_json::Value::String(s) => Some(s),
            _ => None,
        })
        .map(|v: String| match v.as_str() {
            "check_in" => Status::Working,
            "check_out" => Status::CheckedOut,
            _ => Status::Other,
        })
}

pub fn get_status(login_data: &LoginData) -> Status {
    let jst = Utc::now().with_timezone(&Tokyo);
    let mut map = std::collections::BTreeMap::new();
    map.insert("shop_ids".to_owned(), login_data.shop_id.clone());
    map.insert("date".to_owned(), jst.format("%Y-%m-%d").to_string());

    let url = ROOT_URL.to_owned() + "/dashboard/top";

    reqwest::Client::new()
        .post(&url)
        .header("Api-Token", login_data.token.clone())
        .json(&map)
        .send()
        .and_then(|mut response: Response| response.json::<Value>())
        .ok()
        .and_then(|v: Value| parse_info(v))
        .unwrap_or(Status::Other)
}
