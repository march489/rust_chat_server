use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::local::blocking::Client;
// use tokio::test;

use crate::rocket;

const BASE: &str = "/auth";

use super::user::User;

fn get_client() -> Client {
    let diesel_stage: AdHoc = crate::message_handler::stage();
    let login_stage: AdHoc = crate::login::stage();
    match Client::tracked(rocket::build().attach(diesel_stage).attach(login_stage)) {
        Ok(client) => client,
        Err(e) => {
            println!("{:?}", e);
            panic!()
        }
    }
}

#[test]
fn delete_destroys_table_returns_empty() {
    let client: Client = get_client();
    assert_eq!(client.delete(BASE).dispatch().status(), Status::Ok);
    assert_eq!(
        client.get(BASE).dispatch().into_json::<Vec<i32>>(),
        Some(vec![])
    );
}

#[test]
// #[tokio::test]
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
        let new_user: User = User::new(&email, &pw);

        let new_user_id: i32 = client
            .post(BASE)
            .json(&new_user)
            .dispatch()
            .into_json()
            .unwrap();

        // lets test it
        let credentials: User = client
            .get(format!("{}/id/{}", BASE, new_user_id))
            .dispatch()
            .into_json()
            .unwrap();

        assert_eq!(credentials.id.unwrap(), new_user_id);
        assert_eq!(credentials.email, email);
    }
}
