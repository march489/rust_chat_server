use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::local::blocking::Client;
use rocket::response;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Post {
    author: String,
    thread: String,
    body: String,
}

fn _run_test(base: &str, stage: AdHoc) {
    const N: usize = 20;

    let client = Client::tracked(rocket::build().attach(stage)).unwrap();

    assert_eq!(client.delete(base).dispatch().status(), Status::Ok);
    assert_eq!(
        client.get(base).dispatch().into_json::<Vec<i32>>(),
        Some(vec![])
    );

    for i in 1..=N {
        let post: Post = Post {
            author: String::from("MD"),
            thread: String::from("Lobby"),
            body: format!("This is message number{i}"),
        };

        let response = client.post(base).json(&post).dispatch().into_json::<Post>();
        assert_eq!(response.unwrap(), post);

        let list = client.get(base).dispatch().into_json::<Vec<i32>>().unwrap();
        assert_eq!(list.len(), i);

        let last = list.last().unwrap();
        let response = client.get(format!("{}/{}", base, last)).dispatch();
        assert_eq!(response.into_json::<Post>().unwrap(), post);
    }
}

#[test]
fn test_diesel_db() {
    _run_test("/diesel", crate::handler::stage())
}
