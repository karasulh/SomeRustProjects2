mod error;
pub use error::TransactionError;
//make it pub to be seen by main.rs 
use serde_derive::*;


#[derive(Deserialize,Serialize,Debug)]

pub struct Transaction{
    from:String,
    to:String,
    amount: u64,
}


pub fn get_first_transaction_for(fname:&str, uname:&str)-> Result<Transaction,failure::Error>{//Option<Transaction>{
    //let trans = get_transactions_v3(fname).ok()?;
    let trans = get_transactions_v3(fname)?;
    for t in trans{
        if t.from == uname {
            return Ok(t); //return Some(t);
        }
    }
    Err(TransactionError::Mess("Coudlnt find transaction with that name").into())//None
}

pub fn get_transactions(fname:&str) -> Result<Vec<Transaction>,String>{
    //Err("No Trans".to_string());
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    let t:Vec<Transaction> = match serde_json::from_str(&s){
        Ok(v)=>v,
        Err(e)=>return Err(e.to_string()),
    };
    Ok(t)
}

//Avoid using match with combinators, the same behaviour with "get_transactions" functions
pub fn get_transactions_v2(fname:&str) -> Result<Vec<Transaction>,TransactionError>{
    std::fs::read_to_string(fname)
        .map_err(|e| e.into())//because we implement From trait, we can use "into", it converts it automatically.
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.into()))//e.to_string()

}

//Using "?" mark to return error type and result
pub fn get_transactions_v3(fname:&str) -> Result<Vec<Transaction>,TransactionError>{

    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
    //The above line has the same meaning the below lines.
    /*
    Ok(
        match serde_json::from_str(&match std::fs::read_to_string(fname){
            Ok(v)=>v,
            Err(e)=>return Err(e.into())
        }) {
            Ok(v)=>v,
            Err(e)=>return Err(e.into())
        }
    )
    */
}
