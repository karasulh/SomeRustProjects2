use std::{sync::{Arc,mpsc,Mutex}, time::Duration};

pub struct ThreadPool{
    ch_s:Option<mpsc::Sender<Box<dyn Fn()+Send>>>,
    n:u32,
    ch_done : mpsc::Receiver<()>
}

impl ThreadPool{
    pub fn new(n:u32) -> Self{
        let (ch_s,ch_r) = mpsc::channel();
        let a = Arc::new(Mutex::new(ch_r));

        let (ch_done_s,ch_done) = mpsc::channel();

        for _ in 0..n{
            let a2 = a.clone();
            let ch_done_s2 = ch_done_s.clone();
            std::thread::spawn(move || loop {
                let m = a2.lock().unwrap();
                let f : Box<dyn Fn()+Send> = match m.recv() {
                    Ok(f) => f,
                    Err(_) => {
                        ch_done_s2.send(()).ok();
                        return;
                    }
                };  
                drop(m); //if we dont drop mutex, then at the same time only 1 function runs.
                f();
            });
        }
        ThreadPool { ch_s:Some(ch_s), n, ch_done }
    }

    pub fn run<F:Fn()+Send+'static>(&self,f:F){
        if let Some(ref ch_s) = self.ch_s{
            ch_s.send(Box::new(f)).unwrap();
        }
    }

    pub fn wait(mut self){
        self.ch_s.take();//Takes value out of Option, leave it as None
        for _ in 0..self.n{
            self.ch_done.recv().unwrap();//wait for done messages to come back for all; else, some are missing, time is not enough to take all messages
        }
        // If we write "for _ in 0..=self.n", we have 1 more recv, then when the all senders are destroyed, it returns Err. Due to unwrap(), it will panic.
    }
}

fn main(){
    let tp = ThreadPool::new(10);
    for i in 0..100{
        tp.run(move || {
            std::thread::sleep(std::time::Duration::from_millis(200));
            println!("run = {}",i);
        })
    }

    //std::thread::sleep(Duration::from_millis(3000));
    tp.wait();//instead ıf thread::sleep
}