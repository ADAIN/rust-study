pub fn run(){
    let a = [0, 1, 2, 3, 4];
    for number in a.iter() {
        println!("number is {}", number);
    }

    for number in 0..5{
        println!("number is {}", number);
    }

    let mut b: Vec<i32> = Vec::new();
    for number in 0..5{
        b.push(number);
    }

    println!("{:?}", b);

    for number in b.iter(){
        println!("{}", number);
    }

    let name = "Jang SungYong";
    for my_char in name.chars(){
        println!("{}", my_char);
    }

    for (i, my_char) in name.chars().enumerate(){
        println!("{}:{}", i, my_char);
    }
}