//#![feature(futures_api)] //For unstable Rust version, not need for stable if this feature already exist
//#![feature(async_await)]

use futures::future::Future;
use futures::task::Context;
use futures::task::Poll;

use std::pin::Pin; //This is a wrapper around a kind of pointer which makes that pointer "pin" its value in place, preventing the value referenced by that pointer from being moved

pub struct SimpleFuture{
    n:i32,
}

impl Future for SimpleFuture{
    type Output = i32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> { //pin is internal pointer, guarantees the object will not move
        Poll::Ready(self.n)
    }
}

pub async fn simpleexec(p:i32)->i32{
    p+10
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use futures::future::FutureExt; //provides more methods and more easy ways on futures
    use futures::channel::oneshot;
    #[test]
    fn test_future_returns_a_value() {
        //let f = SimpleFuture{n:10};
        let f = simpleexec(10);
        //Pin::new(&mut f).poll();
        let (ch_s,ch_r) = oneshot::channel();
        //let v = block_on(f);
        //let v = block_on(f.map(|n|n+1));//block thread until future completion
        block_on(f.map( move |n| ch_s.send(n+5)));
        let result = block_on(ch_r).unwrap();
        //assert_eq!(v, 11);
        //assert_eq!(result, 15);
        assert_eq!(result, 25);
    }
    #[test]
    fn tes_async_send(){
        let (ch_s,ch_r) = oneshot::channel::<i32>();
        block_on(async move {
            let v = simpleexec(10).await; //use await only in async code
            ch_s.send(v);
        });

        let fin = block_on(async move {
            let res = ch_r.await.unwrap();
            res + 5
        });
        assert_eq!(fin,25);
    }
}
