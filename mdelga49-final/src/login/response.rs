use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub authorized: bool,
    pub reason: Option<String>,
}

impl Response {
    pub fn new(auth: bool, reason: Option<String>) -> Option<Response> {
        match auth {
            true => Some(Response {
                authorized: auth,
                reason,
            }),
            false => match reason {
                Some(r) => Some(Response {
                    authorized: false,
                    reason: Some(r),
                }),
                None => None,
            },
        }
    }
}
