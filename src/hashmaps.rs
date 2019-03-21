use std::collections::HashMap;

pub fn run(){
    let mut my_scores = HashMap::new();

    my_scores.insert(String::from("Blue"), 10);
    my_scores.insert(String::from("Blue"), 120);
    my_scores.insert(String::from("Red"), 20);
    my_scores.entry(String::from("Red")).or_insert(1000);
    my_scores.entry(String::from("Yellow")).or_insert(1000);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let init_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
    for score in scores.iter(){
        println!("{}: {}", score.0, score.1);
    }

    println!("{}", my_scores.get(&String::from("Blue")).unwrap());

    for (key, value) in my_scores {
        println!("{}: {}", key, value);
    }

    let text = "hello 안녕 안녕 안녕";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}