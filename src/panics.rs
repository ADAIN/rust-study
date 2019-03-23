use std::fs::{File};
use std::io::{ErrorKind, Read, Write};

pub fn run(){
    //panic!("패닉입니다.");
    //let v = vec![0, 1, 2];
    //v[100];

    let file_name = "hello.txt";
//    let f = File::open(file_name);
//    let f = match f {
//        Ok(file) => file,
//        Err(ref error) if error.kind() == ErrorKind::NotFound =>{
//            match File::create(file_name) {
//                Ok(fc) => fc,
//                Err(error) => {
//                    panic!("Tried to create file but there was a problem: {:?}", error);
//                },
//            }
//        },
//        Err(error) => {
//            panic!(
//                "There was a problem opening the file: {:?}",
//                error
//            )
//        },
//    };

    //let f = File::open(file_name).unwrap();
    //let f = File::open(file_name).expect("파일 없음");
    let file_string = match read_username_from_file(file_name) {
        Ok(res) => res,
        Err(err) => {
            panic!("에러!!!! {:?}", err);
        }
    };

    println!("{}", file_string);
}

fn read_username_from_file(file_name: &str) -> Result<String, std::io::Error> {
    let f = File::open(file_name);
    // 숏컷 없는 버젼
//    let mut f = match f {
//        Ok(fc) => fc,
//        Err(ref e) if e.kind() == ErrorKind::NotFound =>{
//          match File::create(file_name) {
//              Ok(mut fc) => {
//                  fc.write_all(String::from("god from heaven (GFH)").as_bytes())?;
//                  match File::open(file_name) {
//                      Ok(fc) => fc,
//                      Err(e) => panic!("Wow {:?}", e),
//                  }
//              },
//              Err(e) => return Err(e),
//          }
//        },
//        Err(e) => return Err(e),
//    };
    // 숏컷 사용 버전
    let mut f = match f {
        Ok(fc) => fc,
        Err(ref e) if e.kind() == ErrorKind::NotFound =>{
            let mut file = File::create(file_name)?;
            file.write_all(String::from("신").as_bytes())?;
            File::open(file_name)?
        }, Err(e) => return Err(e),
    };

    let mut s = String::new();
//    match f.read_to_string(&mut s) {
//        Ok(_) => Ok(s),
//        Err(e) => Err(e),
//    }

    f.read_to_string(&mut s)?;
    Ok(s)
}