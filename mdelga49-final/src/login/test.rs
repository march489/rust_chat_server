use rocket::http::Status;
use rocket::local::blocking::Client;

use crate::db::Db;
use crate::login::response::Response;

const BASE: &str = "/auth";
const NO_MATCH_ERR_MSG: &str = "Invalid username or password";

use super::user::User;

fn get_client() -> Client {
    let rocket = rocket::build()
        .attach(Db::fairing())
        .attach(crate::login::stage());

    Client::tracked(rocket).expect("valid rocket instance")
}

#[test]
fn post_creates_users() {
    let client: Client = get_client();

    // clean the client before testing.
    assert_eq!(client.delete(BASE).dispatch().status(), Status::Ok);

    let mut ids: Vec<i32> = vec![];
    const N: usize = 10;

    for i in 1..=N {
        let new_user: User = User::new(
            &format!("mdelga49{}@depaul.edu", i),
            &format!("password{}!", i),
            &"Marcello".to_string(),
        );
        let new_user_id: i32 = client
            .post(BASE)
            .json(&new_user)
            .dispatch()
            .into_json()
            .unwrap();

        ids.push(new_user_id);
    }

    let all_user_ids: Vec<i32> = client.get(BASE).dispatch().into_json::<Vec<i32>>().unwrap();

    for i in 0..N {
        assert_eq!(ids[i], all_user_ids[i]);
    }

    // teardown
    assert_eq!(client.delete(BASE).dispatch().status(), Status::Ok);
}

#[test]
fn get_users_by_email() {
    let client: Client = get_client();

    // clean the client before testing.
    assert_eq!(client.delete(BASE).dispatch().status(), Status::Ok);

    const N: usize = 10;

    for i in 1..=N {
        let email: String = format!("mdelga59{}@depaul.edu", i);
        let pw: String = format!("password{}!", i);
        let name: String = String::from("Marcello");
        let new_user: User = User::new(&email, &pw, &name);

        let new_user_id: i32 = client
            .post(BASE)
            .json(&new_user)
            .dispatch()
            .into_json()
            .unwrap();

        // lets test it
        let user_info: User = client
            .get(format!("{}/id/{}", BASE, new_user_id))
            .dispatch()
            .into_json()
            .unwrap();

        assert_eq!(user_info.id.unwrap(), new_user_id);
        assert_eq!(user_info.email, email);
    }
}

#[test]
fn created_users_are_authorized() {
    let client: Client = get_client();

    // clean the client before testing.
    assert_eq!(client.delete(BASE).dispatch().status(), Status::Ok);

    let new_user: User = User::new(
        &"mdelgado125@cps.edu".to_string(),
        &"password1".to_string(),
        &"Marcello".to_string(),
    );

    // create the new user
    let new_user_id: i32 = client
        .post(BASE)
        .json(&new_user)
        .dispatch()
        .into_json()
        .unwrap();

    // see if the user is validated
    let response: Option<Response> = client
        .post(format!("{}/auth", BASE))
        .json(&new_user)
        .dispatch()
        .into_json()
        .unwrap();

    assert!(response.is_some());
    let resp = response.unwrap();
    assert!(resp.authorized);
    assert_eq!(resp.id.unwrap(), new_user_id);
}

#[test]
fn incorrect_passwords_are_rejected() {
    let client: Client = get_client();

    // clean the client before testing.
    assert_eq!(client.delete(BASE).dispatch().status(), Status::Ok);

    let mut new_user: User = User::new(
        &"mdelgado125@cps.edu".to_string(),
        &"password1".to_string(),
        &"Marcello".to_string(),
    );

    // create the new user
    let _: i32 = client
        .post(BASE)
        .json(&new_user)
        .dispatch()
        .into_json()
        .unwrap();

    new_user.password = "password2".to_string();

    // check if the new incorrect credentials are authorized
    let response: Option<Response> = client
        .post(format!("{}/auth", BASE))
        .json(&new_user)
        .dispatch()
        .into_json()
        .unwrap();

    assert!(response.is_some());
    let resp = response.unwrap();
    assert!(!resp.authorized);
    assert!(resp.id.is_none());
    assert!(resp.reason.is_some());
    assert_eq!(resp.reason.unwrap(), NO_MATCH_ERR_MSG);
}

#[test]
fn non_users_are_not_authorized() {
    let client: Client = get_client();

    // clean the client before testing.
    assert_eq!(client.delete(BASE).dispatch().status(), Status::Ok);

    let new_user: User = User::new(
        &"mdelgado125@cps.edu".to_string(),
        &"password1".to_string(),
        &"Marcello".to_string(),
    );

    // test if we can get in with credentials that haven't been added to the db
    let response: Option<Response> = client
        .post(format!("{}/auth", BASE))
        .json(&new_user)
        .dispatch()
        .into_json()
        .unwrap();

    assert!(response.is_some());
    let resp = response.unwrap();
    assert!(!resp.authorized);
    assert!(resp.id.is_none());
    assert!(resp.reason.is_some());
    assert_eq!(resp.reason.unwrap(), NO_MATCH_ERR_MSG);
}
