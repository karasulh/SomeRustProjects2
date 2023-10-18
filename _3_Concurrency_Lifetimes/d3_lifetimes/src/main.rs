#[derive(Debug)]
pub struct StringHolder<'a>{
    s:&'a str,
    p:&'a str,
}


fn main() {
    println!("Hello, world!");

    let mut s = make_str(7);
    
    s = to_people(s);  
    to_frogs(&mut s);
    let p =part(&s);
    //s.push_str("p"); //cannot borrow s mutable because it is already borrowed as immutable
    
    println!("p = {}",p);
    println!("s = {}",s);

    let tog = two_strings(p, &s);
    println!("Tog = {:?}",tog);//If this line is at the below of s.push_str, it gives error as before 
    s.push_str(" anything");
    println!("final s = {}",s);
}

fn to_people(mut s:String) -> String{
    s.push_str(" people");
    s
}

fn to_frogs(s: &mut String ){
    s.push_str(" frogs");
}

fn make_str(n:i32) -> String {
    format!("hello {}",n)
}

//It gives error because we create a local variable in function in stack, we couldnot return its reference to outside 
//because at the end of function, it is killed. So, return String or do it like below function "part"
/*
fn make_str_vfalse(n:i32) -> &'static str {
    let s = format!("hello {}",n);
    &s
}
*/

//fn part<'a>(s:&'a str) -> &'a str {
fn part(s:&str) -> &str {
    if s.len() > 4{
        &s[0..4]
    }
    else {
        s
    }
}
 

pub fn two_strings<'a>(s: &'a str, p: &'a str) ->StringHolder<'a>{
    StringHolder { s, p }
}
