use std::time::Duration;
use std::sync::{Arc,Mutex};

fn main(){
    with_arc();
    with_channels();
}

//Arc and mutex provides that the data can be used between threads safely and can be mutated.
//Arc: object is safely used between threads
//Mutex: object can be mutated in threads by locking

fn with_arc(){
    let m = Arc::new(Mutex::new(String::from("moving"))); //i32 can be copied so it will not be a problem between threads;
    let m2 = m.clone();
    std::thread::spawn(move ||{ //closure may outlive function and we cant borrow m, so use move
        println!("this is the new thread");
        let mut s = m2.lock().unwrap();
        s.push_str(" on the new thread");
        println!("m = {}",s);
    });
    std::thread::sleep(Duration::from_millis(1000));
    println!("this is the initial thread");
    let s = m.lock().unwrap();
    println!("now m = {}",s);
}

fn with_channels(){
    let (ch_s,ch_r) = std::sync::mpsc::channel::<Box<dyn Fn(&mut String)+Send>>();

    let (done_s,done_r) = std::sync::mpsc::channel::<()>();//crete a channel for empty tuple

    std::thread::spawn(move ||{
        let mut hidden = String::new();
        loop{
            match ch_r.recv(){
                Ok(f)=> {
                    f(&mut hidden);
                    println!("hidden = {}",hidden);
                }
                Err(e) => {
                    println!("Done");
                    done_s.send(()).unwrap();
                    return;
                }
            }
        }
    });

    ch_s.send(Box::new(|s: &mut String|{s.push_str("Hello");})).unwrap();

    let ch2 = ch_s.clone();

    //ch_s.send(Box::new(|s: &mut String|{s.push_str(" world");})).unwrap();
    ch2.send(Box::new(|s: &mut String|{s.push_str(" world");})).unwrap();

    drop(ch_s);
    drop(ch2);//call Err of ch_w.recv() if there is no sender, then it triggers bottom line

    done_r.recv().ok();//By putting this empty tuple sender receiver, we dont need to sleep to wait finishing thread, when the ch_s thread finishs we will know thanks to this.
    //std::thread::sleep(Duration::from_millis(1000));
}