use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub authorized: bool,
    pub id: Option<i32>,
    pub reason: Option<String>,
}

impl Response {
    pub fn new(auth: bool, id: Option<i32>, reason: Option<String>) -> Option<Response> {
        match auth {
            true => Some(Response {
                authorized: auth,
                id,
                reason,
            }),
            false => match reason {
                Some(r) => Some(Response {
                    authorized: false,
                    id: None,
                    reason: Some(r),
                }),
                None => None,
            },
        }
    }
}
