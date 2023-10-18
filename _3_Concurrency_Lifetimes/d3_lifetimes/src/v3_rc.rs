use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct WithLife<'a>{
    s: &'a String,
}

#[derive(Debug)]
pub struct NoLife{
    s:Rc<RefCell<String>>
}

/*
fn make_with_life<'a>(fname:&str) -> Result<(WithLife<'a>,WithLife<'a>),std::io::Error>{
    let s = std::fs::read_to_string(fname)?;
    Ok((WithLife{s:&s},WithLife{s:&s})) //Gives Error: Couldnot return reference of local variable s
                                        //How are we solving these cases? We will use "Rc"
}
*/

fn make_no_life(fname:&str) -> Result<(NoLife,NoLife),std::io::Error>{
    let s = std::fs::read_to_string(fname)?;
    let r = Rc::new(RefCell::new(s));
    Ok((NoLife{s:Rc::clone(&r)},NoLife{s:r})) //Gives Error: Couldnot return reference of local variable s
                                        //How are we solving these cases? We will use "Rc"
    
    //Rc is not mutable. Refcell provides mutability. Refcell internally guards borrow checker and make sure that our changes are correct.
    //Externally, it pretends it doesnot change so to the borrow checker, it is an immutable object but to the contents it doesnot.
    //Refcell act as runtime guard against two mutable references existing for the same object at the same time.
    //Refcell is checking whether it has been borrowed or not.
    //But for multithreads, use atomic Rc instead of Rc and use mutex instead of Refcell
}

fn main() -> Result<(), std::io::Error>{
    //let (l1,l2) = make_with_life("test_data/v3_data.txt")?;
    let (n1,n2) = make_no_life("test_data/v3_data.txt")?;

    let mut s = n1.s.borrow_mut();
    s.push_str("What if it was even bigger");

    //let s2 = n2.s.borrow();//It gives runtime BorrowError thanks to Refcell control. There must not be mutable borrow and immutable borrow at the same time.
    //println!("s2 = {}",s2);

    println!("n1 = {:?}",n1); //Refcell doesnot give internal data info if it is borrowed :  n1 = NoLife { s: RefCell { value: <borrowed> } }
    println!("n2 = {:?}",n2);

    println!("s = {:?}",s);

    drop(s);

    println!("n1 = {:?}",n1); //Refcell gives internal data now.
    println!("n2 = {:?}",n2);

    Ok(())
}