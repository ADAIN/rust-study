//! Author : SungYong Jang, jsy@adain.kr
//! Date :  2019-05-10
//! Description :
//!

struct Context<'a>(&'a str);
struct Parser<'a, 'b: 'a> {
    context: &'a Context<'b>,
}
struct Ref<'a, T: 'a>(&'a T);
struct StaticRef<T: 'static>(&'static T);

trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

impl<'a, 'b> Parser<'a, 'b> {
    fn parse(&self) -> Result<&'b str, &'b str> {
        //Ok(&self.context.0)
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<&str, &str> {
    Parser { context: &context }.parse()
}

pub fn run(){
    let context = Context("HI!!!");
    let res = parse_context(context);

    match res {
        Ok(message) => println!("OK: {}", message),
        Err(message) => println!("ERR: {}", message),
    }

    let num = 5;
    let obj = Box::new(Ball {diameter: &num}) as Box<Red>;
}