pub fn run(){
    let x = 5;
    let y = x;
    println!("{}:{}", x, y);

    let n1 = "abc";
    let n2 = n1;
    println!("{}:{}", n1, n2);

    let a = String::from("HI");
    let b = a;
    println!("{}", b);

    let name = String::from("JSY");
    let name = hi(name);
    println!("{}", name);

    hi2(&name);
    hi2(&name);
    hi2(&name);

    let mut my_data = String::from("NICE OK1OK ");
    let slice = &my_data[0..2];
    println!("{}", slice);
    add_comma(&mut my_data);
    add_comma(&mut my_data);
    add_comma(&mut my_data);
    println!("{}", my_data);
    println!("{}", first_word(&my_data));
}

fn hi(name: String) -> String{
    println!("HI, {}", name);
    name
}

fn hi2(name: &String) {
    println!("HI2, {}", name);
}

fn add_comma(my_str: &mut String){
    my_str.push_str(",");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //0..i == ..i
            return &s[..i];
        }
    }

    &s[..]
}