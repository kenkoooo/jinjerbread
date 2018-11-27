#[derive(Deserialize, Debug, PartialEq)]
pub struct LoginResult {
    pub code: usize,
    pub message: String,
    pub data: LoginData,
}

#[derive(Deserialize, Debug, PartialEq)]
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

#[derive(Debug)]
pub enum Status {
    Working,
    CheckedOut,
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_test() {
        assert_eq!(
            serde_json::to_string(&TimeCard::new(Check::In)).unwrap(),
            r#"{"comment":"","night_mode":"0","type":"check_in","type_swipe":"PC"}"#
        );
        assert_eq!(
            serde_json::to_string(&LoginInfo {
                company_code: 1234,
                email: "mail@example.com".to_owned(),
                password: "password".to_owned(),
                remember_me: true
            })
            .unwrap(),
            r#"{"company_code":"1234","password":"password","email":"mail@example.com","remember_me":true}"#
        );
    }

    #[test]
    fn deserialize_test() {
        let json = r#"
        {
            "code":200,
            "message":"msg",
            "data":{
                "id": "id",
                "username": "username",
                "name": "name",
                "token": "token",
                "php_token": "php_token",
                "shop_id": "shop_id",
                "shop_name": "shop_name",
                "company_id": "company_id",
                "max_break_time": 100
            }
        }"#;
        assert_eq!(
            serde_json::from_str::<LoginResult>(json).unwrap(),
            LoginResult {
                code: 200,
                message: "msg".to_owned(),
                data: LoginData {
                    id: "id".to_owned(),
                    username: "username".to_owned(),
                    name: "name".to_owned(),
                    token: "token".to_owned(),
                    php_token: "php_token".to_owned(),
                    shop_id: "shop_id".to_owned(),
                    shop_name: "shop_name".to_owned(),
                    company_id: "company_id".to_owned(),
                    max_break_time: 100
                }
            }
        );
    }
}
