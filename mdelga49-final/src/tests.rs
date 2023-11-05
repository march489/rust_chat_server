use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::local::blocking::Client;

use crate::post::Post;

fn _run_test(base: &str, stage: AdHoc) {
    const N: usize = 5;

    let client = Client::tracked(rocket::build().attach(stage)).unwrap();

    assert_eq!(client.delete(base).dispatch().status(), Status::Ok);
    assert_eq!(
        client.get(base).dispatch().into_json::<Vec<i32>>(),
        Some(vec![])
    );

    for i in 1..=N {
        let mut post: Post = Post::new("MD", "Lobby", format!("This is post {i}").as_str());

        println!("Trial {i}:");
        let response = client
            .post(base)
            .json(&mut post)
            .dispatch()
            .into_json::<Post>();

        let response_id: i32 = response.unwrap().id.unwrap();

        let list = client.get(base).dispatch().into_json::<Vec<i32>>().unwrap();
        assert_eq!(list.len(), i);

        let last = list.last().unwrap();
        assert_eq!(*last, response_id);

        let response: Post = client
            .get(format!("{}/{}", base, last))
            .dispatch()
            .into_json::<Post>()
            .unwrap();

        // println!("response json:\n{:?}", response);
        assert_eq!(response.author, post.author);
        assert_eq!(response.thread, post.thread);
        assert_eq!(response.body, post.body);
    }
}

#[test]
fn test_diesel_db() {
    _run_test("/diesel", crate::handler::stage())
}
