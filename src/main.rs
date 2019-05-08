//mod number_guess_game;
//mod for_loops;
//mod ownership;
//mod structs;
//mod enums;
//mod matches;
//mod vectors;
//mod strings;
//mod hashmaps;
//mod panics;
//mod generics;
//mod lifetime;
//mod closure;
//mod iter;
//mod concurrent;
//mod mutex;
//mod oop;
extern crate rust_study;
use rust_study::Post;

fn main() {
    //number_guess_game::run();
    //for_loops::run();
    //ownership::run();
    //structs::run();
    //enums::run();
    //matches::run();
    //vectors::run();
    //strings::run();
    //hashmaps::run();
    //panics::run();
    //generics::run();
    //lifetime::run();
    //closure::run();
    //iter::run();
    //concurrent::run();
    //concurrent::channel_run();
    //mutex::run();
    //oop::run();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
