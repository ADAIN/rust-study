//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-05-08
//! Description :
//!
extern crate rust_study;
use rust_study::Post;
use rust_study::AveragedCollection;

pub fn run(){
    let mut averaged_collection = AveragedCollection::new();
    println!("average = {}", averaged_collection.average());
    averaged_collection.add(100);
    averaged_collection.add(100);
    averaged_collection.add(200);
    averaged_collection.add(200);
    averaged_collection.add(90);
    averaged_collection.add(90);
    averaged_collection.add(90);
    println!("average = {}", averaged_collection.average());
    averaged_collection.remove();
    averaged_collection.remove();
    averaged_collection.remove();
    println!("average = {}", averaged_collection.average());

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}