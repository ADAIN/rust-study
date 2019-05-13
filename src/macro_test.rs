//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-05-13
//! Description :
//!
#[macro_export]
macro_rules! m_vec {
    ( $($x: expr), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


pub fn run() {
    let v = m_vec![1, 2];
    println!("{:?}", v);
}
