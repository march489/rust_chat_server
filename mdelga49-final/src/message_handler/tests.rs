use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::local::blocking::Client;

use crate::message_handler::post::Post;

fn _run_test(base: &str, stage: AdHoc) {
    const N: usize = 20;

    let client = Client::tracked(rocket::build().attach(stage)).unwrap();

    // NOTE This is not threadsafe
    assert_eq!(client.delete(base).dispatch().status(), Status::Ok);
    assert_eq!(
        client.get(base).dispatch().into_json::<Vec<i32>>(),
        Some(vec![])
    );

    for i in 1..=N {
        let mut post: Post = Post::new("MD", "Lobby", format!("This is post {i}").as_str());

        let response = client
            .post(base)
            .json(&mut post)
            .dispatch()
            .into_json::<Post>();

        let response_id: i32 = response.unwrap().id.unwrap();

        let list: Vec<i32> = client.get(base).dispatch().into_json::<Vec<i32>>().unwrap();
        assert_eq!(list.len(), i);

        let last: &i32 = list.last().unwrap();
        assert_eq!(*last, response_id);

        let response: Post = client
            .get(format!("{}/{}", base, last))
            .dispatch()
            .into_json::<Post>()
            .unwrap();

        // println!("response json:\n{:?}", response);
        assert_eq!(response.user_id, post.user_id);
        assert_eq!(response.thread, post.thread);
        assert_eq!(response.body, post.body);
    }

    for _ in 1..=N {
        let id_list: Vec<i32> = client.get(base).dispatch().into_json::<Vec<i32>>().unwrap();
        assert!(id_list.len() > 0);

        let id: &i32 = id_list.get(0).unwrap();
        let response = client.delete(format!("{}/{}", base, id)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    let list: Vec<i32> = client.get(base).dispatch().into_json::<Vec<i32>>().unwrap();
    assert!(list.is_empty());
}

#[test]
fn test_diesel_db() {
    _run_test("/diesel", crate::message_handler::stage())
}
