
pub fn run(){
    let mut s = String::new();
    let mut data = "initialize";
    let string_from_data = data.to_string();

    s.push_str("GOOD");
    s.push('!');

    println!("{}", data);

    data = "NICE";
    println!("{}", data);

    println!("{}", s);
    println!("{}", &string_from_data[2..]);
    println!("{}", string_from_data.to_owned() + "!");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);

    println!("{}", s2);

    let a = String::from("안녕");
    let b = String::from("하셈");
    let c = a + &b;

    println!("{}", c);
    println!("{}", b);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    let s = format!("{}-{}-{}", "tic", s2, s3);
    println!("{}", s);

    println!("{}", "hi".len());
    println!("{}", "안녕".len());
    println!("{}", &"안녕"[0..3]);

    let hello = "안녕하세요.";
    for c in hello.chars(){
        println!("{}", c);
    }
    for c in hello.bytes(){
        println!("{}", c);
    }
    println!("{}", hello.chars().nth(0).unwrap());
    println!("{}", hello.chars().nth(1).unwrap());
    println!("{}", hello.chars().nth(2).unwrap());
    println!("{}", hello.chars().nth(3).unwrap());
}