use crate::model::{Check, CheckResult, LoginInfo, LoginResult, TimeCard};

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

pub fn check(login_result: &LoginResult, check_kind: Check) -> Result<CheckResult, reqwest::Error> {
    let url = format!(
        "{}/dashboard/shops/{}/time_cards",
        ROOT_URL, login_result.data.shop_id
    );
    let time_card = TimeCard::new(check_kind);
    reqwest::Client::new()
        .post(&url)
        .header("Api-Token", login_result.data.token.clone())
        .json(&time_card)
        .send()
        .and_then(|mut result| result.json::<CheckResult>())
}
