#![allow(dead_code)]
#![allow(unused_imports)]
use futures::channel::mpsc;
use futures::StreamExt;
use futures::executor::{self, ThreadPool};
use std::iter::Iterator;

use std::{
    future::Future, 
    pin::Pin, 
    sync::{Arc, Mutex}, 
    task::{Context, Poll, Waker}, 
    thread, 
    time::Duration
};

use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use futures::{future::{FutureExt, BoxFuture}, task::{ArcWake, waker_ref}};
// use super::timefuture::*;

use chrono::prelude::*;

// use std::

fn test01(){
    let pool = ThreadPool::new().expect("error");
    let (tx, mut rx) = mpsc::unbounded::<i32>();

    let fut_values = async {
        println!("start");

        let fut_tx_result = async move{
            (0..100).for_each(|v|{
                tx.unbounded_send(v).expect("send error");
            })
        };

        pool.spawn_ok(fut_tx_result);

        let mut fut_values = vec![];
        // while let Ok(n) = rx.try_next(){
        //     fut_values.push(n);
        // }
        // // let fut_values = rx.map(|v|v*2).collect();

        // fut_values
        // rx.poll().await;
        // for _ in 0..100{
        //     if let Some(n) = rx.next().await{
        //         fut_values.push(n);
        //     }
        // }
        while let Some(n) = rx.next().await{
            fut_values.push(n);
        }

        fut_values
        // rx.next().await;
    };

    let values = executor::block_on(fut_values);
    println!("result: {:?}", values);
}

fn test02(){
    // let pool = ThreadPool::new().expect("error");
    let (tx, mut rx) = mpsc::unbounded::<i32>();

    let fut_values = async {
        println!("start");

        let fut_tx_result = async move{
            let _ = tx.unbounded_send(1000);
        };

        // pool.spawn_ok(fut_tx_result);
        executor::block_on(fut_tx_result);

        let ret = rx.next();
        ret.await
    };

    let values = executor::block_on(fut_values);
    println!("result: {:?}", values);
}

fn test03(){
    let pool = ThreadPool::new().expect("Failed to build pool");
    let (tx, rx) = mpsc::unbounded::<i32>();

    let fut_values = async {
        let fut_tx_result = async move {
            (0..100).for_each(|v| {
                tx.unbounded_send(v).expect("Failed to send");
            })
        };

        // Use the provided thread pool to spawn the generated future
        // responsible for transmission
        pool.spawn_ok(fut_tx_result);

        let fut_values = rx
            .map(|v| v * 2)
            .collect();

        // Use the executor provided to this async block to wait for the
        // future to complete.
        fut_values.await
    };

    let values: Vec<i32> = executor::block_on(fut_values);
    println!("Values={:?}", values);
}

fn test04(){
    // let sink = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..10{
        let c = Arc::clone(&sink);
        let handle = std::thread::spawn(move || {
            let mut v = c.lock().unwrap();
            (*v).push(i);
            // *num += 1;
        });
        handles.push(handle);
    }

    for h in handles{
        h.join().unwrap();
    }

    println!("{:?}", sink.lock().unwrap());
    println!("{}", sink.lock().unwrap().len())
}

// use parity_scale_codec::{Decode, Encode};

// #[derive(Decode, Encode)]
// struct Point{
//     x: u32,
//     y: f32,
// }

// fn test05(){
//     let p1 =  Point{x:21, y:1.0};
//     let result = p1.encode();
//     println!("{:?}", result);
// }
// use std::hash::Hash;

// #[derive(Debug, Encode, Decode)]
// struct Point{
//     x: u32,
//     y: u32
// }

// impl PartialEq for Point{
//     fn eq(&self, other: &Self)->bool{
//         self.x == other.x
//     }
// }

fn test06(){
    // let p1 = Point{x:10, y: 10};
    // let p2 = Point{x:12, y: 8};

    // println!("{:?}", p1);
    // println!("{:?}", p2);

    // let p1 = Point{x: 10, y: 12};
    // let p2 = Point{x: 10, y: 15};

    // if p1 == p2{
    //     println!("eq");
    // }
    // else{
    //     println!("ne");
    // }

    // if p1 == p2{
    //     println!("eq");
    // }
    // else{
    //     println!("ne");
    // }

    // if p1 > p2{
    //     println!("p1 > p2");
    // }
    // else{
    //     println!("p1 < p2");
    // }

    // println!("{:?}", p1.hash());
}

use std::ops::Deref;

struct DerefExampleA<T>{
    int_value: u32,
    gen_value: T,
}

impl<T> Deref for DerefExampleA<T>{
    type Target = T;

    fn deref(&self)->&Self::Target{
        &self.gen_value
    }
}

struct DerefExampleB<T>{
    int_value: u32,
    gen_value: T
}

impl<T> Deref for DerefExampleB<T>{
    type Target = u32;

    fn deref(&self)->&Self::Target{
        &self.int_value
    }
}

fn test07(){
    let a = DerefExampleA{int_value: 10u32, gen_value: 0.5f32};
    let b = DerefExampleB{int_value: 10u32, gen_value: 0.5f32};

    println!("{:?}, {:?}", *a, a.deref());
    println!("{:?}, {:?}", *b, b.deref());
}


struct FutStruct{
    start_ms: i64,
}

impl FutStruct{
    pub fn new()->Self{
        Self{
            start_ms: Local::now().timestamp_millis(),
        }
    }
}

impl Future for FutStruct{
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>)->Poll<Self::Output>{
        let cur_ms = Local::now().timestamp_millis();
        if cur_ms > self.start_ms + 10000{
            println!("Ready");
            Poll::Ready(())
        }
        else{
            println!("Pending");
            Poll::Pending
        }
    }
}

// #[derive(Default)]
struct FutStruct2{
    count: u32,
    name: String
}

impl Future for FutStruct2{
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>)->Poll<Self::Output>{
        match self.count{
            8 => Poll::Ready(8),
            _ => {
                println!("pending: {}", self.name);
                self.count += 1;
                // ctx.waker.wake();
                ctx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

async fn test08(){
    let f1 = FutStruct2{count: 0u32, name: "A".into()};
    let f2 = FutStruct2{count: 0u32, name: "B".into()};
    // let f1 = FutStruct2::default();
    // let f2 = FutStruct2::default();
    let (r1, r2) = futures::join!(f1, f2);
    println!("{}, {}", r1, r2);
    // println!("{}", f1.await);
}

pub struct TimerFuture{
    shared_state: Arc<Mutex<SharedState>>
}

struct SharedState{
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture{
    type Output = ();

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>)->Poll<Self::Output>{
        // println!("attemp lock");
        println!("poll");
        let mut shared_state = self.shared_state.lock().unwrap();
        // println!("get lock");
        if shared_state.completed{
            // println!("Poll::Ready, TimerFuture poll()");
            println!("ready");
            Poll::Ready(())
        }
        else{
            // println!("Poll::Pending, TimerFuture poll()");
            shared_state.waker = Some(ctx.waker().clone());
            // ctx.waker().wake_by_ref();
            println!("pending");
            Poll::Pending
        }
    }
}

impl TimerFuture{
    pub fn new(_duration: Duration)->Self{
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();

        thread::spawn(move||{
            thread::sleep(Duration::new(2, 0));

            let mut shared_state = thread_shared_state.lock().unwrap();
            if let Some(waker) = shared_state.waker.take(){
                waker.wake();
            }

            thread::sleep(Duration::new(2, 0));
            shared_state.completed = true;

            // if let Some(waker) = shared_state.waker.take(){
            //     waker.clone().wake();
            // }

            // for _ in 0..3{
            //     {
            //         let mut shared_state = thread_shared_state.lock().unwrap();
            //         if let Some(waker) = shared_state.waker.take(){
            //             waker.clone().wake();
            //         }
            //     }
            //     thread::sleep(Duration::new(2, 0));
            // }

        });

        TimerFuture{ shared_state }
    }
}

fn main(){
    // executor::block_on(TimerFuture::new(Duration::new(5, 0)));
    executor::block_on(TimerFuture::new(Duration::new(0, 100)));
    // executor::block_on(test08());
    // test07();
}
