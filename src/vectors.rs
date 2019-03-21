enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn run(){
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    //let third = &v[22]; // panic error
    println!("{}", third);

    let third = v.get(22);
    if let Option::None = third {
        println!("WOW");
    }else{
        println!("HO");
    }

    let mut a = vec![1, 2, 3, 4, 5];
    a[0] = 1000;
    a.push(10);

    for i in &mut a {
        *i += 100;
    }

    println!("third {:?}", third);

    for i in a {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("HI")),
        SpreadsheetCell::Float(10.12),
    ];

    for row_item in row {
        match row_item {
            SpreadsheetCell::Int(value) => println!("{}", value),
            SpreadsheetCell::Text(value) => println!("{}", value),
            SpreadsheetCell::Float(value) => println!("{}", value),
        }
    }
}