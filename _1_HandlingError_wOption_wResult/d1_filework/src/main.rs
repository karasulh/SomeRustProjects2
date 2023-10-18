//extern crate d1_filework; //No need
use d1_filework::*;
//mod error; //use error::TransactionError; //No need because lib.rs is "publicly use error"
use failure::Error;


fn main() -> Result<(),Error>{ //TransactionError
    println!("Hello, world!");
    let trans = get_transactions_v2("test_data/transactions.json").expect("Couldnot load transactions");
    for t in trans{
        println!("{:?}",t)
    }

    //let t = get_first_transaction_for("test_data/transactions.json", "Matt").ok_or(TransactionError::Mess("couldnot get first transaciton"))?;
     //println!("Matt's first transactions: {:?}",t);

    let t = get_first_transaction_for("test_data/transactions.json", "Mate");
    match t{
        Ok(v) => println!("Found transaction: {:?},",v),
        Err(e) => println!("Error {}, Backtrace = : {}",e,e.backtrace()),
    }
   

    Ok(())
}


