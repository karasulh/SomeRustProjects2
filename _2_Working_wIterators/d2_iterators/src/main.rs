fn main() {
    print_a(&vec!["hello".to_string(), "world".to_string()]);
    print_b(&["hello", "world"]);
    print_c(vec!["hello".to_string(), "world".to_string()].into_iter());

    print_any(vec!["hello", "world"]);
    print_any(&vec!["hello".to_string(), "mars".to_string()]);
    print_any(vec!["hello".to_string(), "mars".to_string()]);
    print_any(&["hello".to_string(), "venus".to_string()]);
    print_any((&["hello", "pluto"]).into_iter());
}

fn print_a(v: &Vec<String>){ //Take reference not to consume vector in loop
    println!("a");
    for (i,val) in v.into_iter().enumerate(){
        println!("{} == {}",i,val);
    }
}

fn print_b(v: &[&str]){
    println!("b");
    for (i,val) in v.into_iter().enumerate(){
        println!("{} == {}",i,val);
    }
}

fn print_c<I:Iterator<Item=String>>(v: I){//For String
    println!("c");
    for (i,val) in v.into_iter().enumerate(){
        println!("{} == {}",i,val);
    }
}

//It is generic version of them above
//It includes printing for both &str and String via AsRef<str>
fn print_any<S:AsRef<str>, I:IntoIterator<Item = S>>(v: I){
    println!("any");
    for (i,val) in v.into_iter().enumerate(){
        println!("{} == {}",i,val.as_ref());
    }
}
