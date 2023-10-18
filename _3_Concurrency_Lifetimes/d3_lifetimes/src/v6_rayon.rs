//rayon enables multithreading, make it easy
//rayon converts iterator to parallel iterator
//rayon join: place one job to queue, start another jıb. If another Rayon thread is available, then it takes a job from that queue. 
use rayon::{join, prelude::{IntoParallelRefMutIterator, ParallelIterator}};
use std::time::Duration;

pub fn on_each<T,F>(v:&mut [T], f:F)
where 
    F:Fn(&mut T)+Send+Copy+Sync, 
    T:Send,
{
    match v.len(){
        0=> return,
        n if n<4 => {
            for i in v {
                f(i)
            }
        },
        n => {
            let (v1,v2) = v.split_at_mut(n/2);
            join(||on_each(v1, f), ||on_each(v2, f)); 
        }
    }
}

fn main(){
    let mut v = Vec::with_capacity(100);
    for i in 0..100{
        v.push(i);
    }

    on_each(&mut v, |n|{
        println!("doing = {}",n);
        std::thread::sleep(Duration::from_millis(600));
        *n += 5;
    });

    println!("result = {:?}",v);

    v.par_iter_mut().for_each(|n|{ //paralel iterator, it has the same behaviour above.
        println!("now doing {}",n);
        *n *=10;
        std::thread::sleep(Duration::from_millis(600));
    });
    println!("result = {:?}",v);
}