extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::models::*;
use self::diesel_demo::*;

fn main() {
    use self::schema::posts::dsl::*;
    // use self::schema::users::dsl::*;

    let mut connection = establish_connection();
    let results = posts
        .filter(published.eq(1))
        .limit(5)
        .select(Post::as_select())
        .load::<Post>(&mut connection)
        .expect("Error loading posts");

    println!("Displaying {} posts...", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
