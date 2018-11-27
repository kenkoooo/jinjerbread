#[derive(Deserialize, Debug)]
pub struct LoginResult {
    pub code: usize,
    pub message: String,
    pub data: LoginData,
}

#[derive(Deserialize, Debug)]
pub struct LoginData {
    pub id: String,
    pub username: String,
    pub name: String,
    pub token: String,
    pub php_token: String,
    pub shop_id: String,
    pub shop_name: String,
    pub company_id: String,
    pub max_break_time: usize,
}

#[derive(Debug, Deserialize)]
pub struct CheckResult {
    pub code: usize,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct LoginInfo {
    #[serde(serialize_with = "to_string_serialize")]
    pub company_code: usize,
    pub password: String,
    pub email: String,
    pub remember_me: bool,
}

fn to_string_serialize<T: std::string::ToString, S: serde::Serializer>(
    t: &T,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(&t.to_string())
}

#[derive(Debug, Serialize)]
pub struct TimeCard {
    comment: String,
    night_mode: String,

    #[serde(rename = "type")]
    check: Check,
    type_swipe: String,
}

#[derive(Debug)]
pub enum Check {
    In,
    Out,
}

impl serde::Serialize for Check {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            Check::In => serializer.serialize_str("check_in"),
            Check::Out => serializer.serialize_str("check_out"),
        }
    }
}

impl TimeCard {
    pub fn new(check: Check) -> Self {
        TimeCard {
            comment: "".to_owned(),
            night_mode: "0".to_owned(),
            check: check,
            type_swipe: "PC".to_owned(),
        }
    }
}
