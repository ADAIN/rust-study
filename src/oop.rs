//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-05-08
//! Description :
//!

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
}