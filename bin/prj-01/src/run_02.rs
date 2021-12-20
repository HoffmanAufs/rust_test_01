#![allow(dead_code)]
// extern crate futures;

use std::time::{Duration, SystemTime, Instant};
use bytes::Bytes;
use rand::Rng;
use std::thread;

// use futures::prelude::*;

fn test01(){
    let mut rng = rand::thread_rng();

    for _ in 0..10{
        let mut x = rng.gen::<u64>();
        x &= 0xFF;
        println!("{:?}", x);
    }
}

fn test02(){
    let n = 100u64;
    println!("u64: {}", n);

    //
    // u64 -> vec -> u64
    let n_vec = n.to_be_bytes().to_vec();
    println!("{:?}", n_vec);

    let mut recover_slice = [0u8; 8];
    recover_slice.copy_from_slice(&n_vec[..8]);
    println!("{:?}", recover_slice);

    let n = u64::from_be_bytes(recover_slice);
    println!("u64: {}", n);

    //
    // u64 -> bytes
    let bytes_mem = Bytes::from(&b"Hello"[..]);
    println!("{:?}", bytes_mem);


    // let slice = v.as_slice();
    // println!("{:?}", slice);

    // let ref_v = v.as_ref();
    // println!("{:?}", ref_v);
    // let v = n.to_be_bytes();
    // println!("{:?}", v);

    // let x1 = v.as_ref();
    // println!("{:?}", x1);

    // let x2 = &v;
    // println!("{:?}", x2);

    // assert_eq!(x1, x2);

    // // let mem = Bytes::from(&v[..]);
    // let b = b"Hello world";
    // println!("{:?}", b);


    // let mem = Bytes::from(&x[..]);
    // println!("{:?}", mem);

    // let mem = Bytes::from(&v.as_ref()[..]);
    // println!("{:?}", mem);

    // let mut mem = Bytes::from(&b"12345678"[..]);
    // println!("{:?}", mem);

    // let s = mem.as_ref();
    // println!("{:?}", s);
}

fn test03(){
    let mut v = Vec::new();

    v.push(40u64);
    v.push(30u64);
    v.push(20u64);
    v.push(10u64);

    v.sort();
    println!("{:?}", v);

    v.clear();
    println!("{:?}", v);
}

use std::collections::BTreeMap;

fn test04(){
    let mut m :BTreeMap<u64, String>= BTreeMap::new();

    m.insert(100, format!("AAAAAA"));
    m.insert(10, format!("BBB"));
    m.insert(40, format!("CCCCC"));
    m.insert(300, format!("DDD"));

    for i in m.iter().rev(){
        println!("{}, {}", i.0, i.1);
    }

    m.clear();
    println!("{}", m.len());
}

fn test05(){
    let nested_vec: Vec<Vec<i32>> = vec![vec![1,2,3], vec![11,22,33]];
    println!("{:?}", nested_vec);
    let flat_ret  = nested_vec.into_iter().flatten();
    let flat_vec: Vec<i32> = flat_ret.collect();
    println!("{:?}", flat_vec);

    // for i in flat_ret{
    //     println!("{}", i);
    // }
}

fn test06(){
    let v1 = vec![1,2,3,4,5,6,7];
    let ret = v1.iter().all(|n| n%2 == 0);
    println!("{}", ret);
}

fn test07(){
    let r1 :Result<Option<i32>, i32> = Ok(Some(1));
    println!("r1: {:?}", r1);
    let r2 = r1.ok();
    println!("r2 = r1.ok(): {:?}", r2);
    let r3 = r2.flatten();
    println!("r3 = r2.flatten(): {:?}", r3);

    println!("");

    let r1 :Result<Option<i32>, i32> = Err(3);
    println!("r1: {:?}", r1);
    let r2 = r1.ok();
    println!("r2 = r1.ok(): {:?}", r2);
    let r3 = r2.flatten();
    println!("r3 = r2.flatten(): {:?}", r3);
}

fn test08(){
    // let r1 = Some(Some(Some(1)));
    // println!("{:?}", r1.flatten());
    // let r1 = Some(Some(Some(Some(1))));
    // println!("{:?}", r1.flatten());

    // let r1: Option<i32> = None;
    // let r2 = Some(r1);
    // println!("{:?}", r2.flatten());
    let r1 :Option<Option<i32>> = Some(Some(32));
    println!("{:?}", r1.flatten());

    let r1 :Option<Option<i32>> = Some(Some(10));
    println!("{:?}", r1.unwrap());
}

fn delay_fn(tx: oneshot::Sender<i32>, delay: u32){
    thread::sleep_ms(delay);
    let _ = tx.send(100);
}

async fn test09(){
    let start = SystemTime::now();
    println!("start");
    // println!("start: {:?}", start);
    let (tx1, rx1) = oneshot::channel::<i32>();
    let (tx2, rx2) = oneshot::channel::<i32>();

    thread::spawn(move||{
        delay_fn(tx1, 300);
    });

    thread::spawn(move||{
        delay_fn(tx2, 400);
    });

    // select!{
    //     ret = rx1.await => {
    //         println!("rx1: {}", ret);
    //     },
    //     ret = rx2.await => {
    //         println!("rx2: {}", ret);
    //     },
    // };

    // let ret = rx.await;
    // println!("{:?}", ret);
    // let end = SystemTime::now();
    let eplased = start.elapsed();
    println!("finished: {:?}", eplased);
}

async fn test10(){
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(10);

    select!{
        a = a_fut => {
            println!("a: {}", a);
        },
        b = b_fut => {
            println!("b: {}", b);
        },
    }
}

use futures::{channel::oneshot, select, future, channel::mpsc, stream::StreamExt, prelude::*};
use futures_timer::Delay;

async fn test11(){
    let (tx, mut rx) = mpsc::unbounded();

    thread::spawn(move||{
        let mut i = 0;
        // let mut tx = tx.clone();
        loop{
            let _ = tx.unbounded_send(i);
            i += 1;
            thread::sleep_ms(500);
        }

    });
    let mut timer = Delay::new(Duration::from_millis(3000)).fuse();
    
    loop{
        select!{
            _ = timer =>{
                break;
            }
            v = rx.select_next_some().fuse()=>{
                println!("{}", v);
            }
        }
    }
}

async fn test12(){
    let (tx, rx) = mpsc::unbounded();
    let mut r = rx;

    // let (tx, mut rx) = mpsc::channel::<Vec<i32>>(100);

    thread::spawn(move||{
        let mut i = 0;
        // let mut tx = tx.clone();
        loop{
            let _ = tx.unbounded_send(vec![i, i+1, i*2]);
            // let _ = tx.try_send(vec![i+3, i+4, i+5]);
            i += 1;
            thread::sleep_ms(500);
        }

    });
    let mut timer = Delay::new(Duration::from_millis(3000)).fuse();
    
    loop{
        select!{
            _ = timer =>{
                break;
            }

            v = r.select_next_some().fuse()=>{
                println!("{:?}", v);
            }
        }
    }
}

fn propagate_number(n: u64, sender: mpsc::UnboundedSender<Vec<u64>>){
    println!("recieve number: {}", n);
    thread::sleep_ms(2000);
    println!("send_back");
    let _ = sender.unbounded_send(vec![n, n+10, n+20]);
}

async fn test13(){
    let (tx, mut rx) = mpsc::unbounded::<Vec<u64>>();

    propagate_number(10u64, tx);

    // let ret = rx.select_next_some().await;
    let ret = rx.next().await;
    println!("result: {:?}", ret);
}

fn test14(){
    type My64 = u64;

    let x = 100u64;
    let y :My64 = 100;

    if x == y {
        println!("eq");
    }
}

fn test15(){
    let async_fn = async move{
        let (tx, mut rx) = mpsc::unbounded::<Vec<u64>>();

        propagate_number(10u64, tx);

        // let ret = rx.select_next_some().await;
        let ret = rx.next().await;
        println!("result: {:?}", ret);
    };

    futures::executor::block_on(async_fn);
}

use futures::executor::LocalPool;

async fn test17(){
    let t = std::thread::spawn(move || {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen::<u64>() & 0xFFFFu64;
        println!("{}", random_number);
    });

    t.join();
}

async fn test18(){
    let (tx, rx) = mpsc::unbounded();
    let mut r = rx;

    // let (tx, mut rx) = mpsc::channel::<Vec<i32>>(100);

    thread::spawn(move||{
        let mut i = 0;
        // let mut tx = tx.clone();
        loop{
            let _ = tx.unbounded_send(vec![i, i+1, i*2]);
            // let _ = tx.try_send(vec![i+3, i+4, i+5]);
            i += 1;
            thread::sleep_ms(500);
        }

    });
    // let mut timer = Delay::new(Duration::from_millis(3000)).fuse();
    
    // loop{
    //     select!{
    //         _ = timer =>{
    //             break;
    //         }

    //         v = r.select_next_some().fuse()=>{
    //             println!("{:?}", v);
    //         }
    //     }
    // }
    // let start = SystemTime::now();
    // loop{
    //     if let Ok(ret) = r.try_next(){
    //         println!("{}", ret);
    //     }
    // }
}

fn sub_test16(){
    let async_fn = async {
        println!("async start");
        let timer = Delay::new(Duration::from_millis(3000));
        let _ = timer.await;
        println!("async done");
    };

    futures::executor::block_on(async_fn);
}

async fn test16(){
    println!("start t16");

    sub_test16();

    // let _ = async_fn.await;
    // pool.block_on(async {
    // futures::executor::block_on(async_fn);

    println!("t16 done");
}

fn test19(){
    let (tx, rx) = mpsc::unbounded();
    let mut r = rx;

    // let (tx, mut rx) = mpsc::channel::<Vec<i32>>(100);

    thread::spawn(move||{
        let mut i = 0;
        // let mut tx = tx.clone();
        loop{
            let _ = tx.unbounded_send(vec![i, i+1, i*2]);
            // let _ = tx.try_send(vec![i+3, i+4, i+5]);
            i += 1;
            thread::sleep_ms(500);
        }

    });
    // let mut timer = Delay::new(Duration::from_millis(3000)).fuse();

    let mut i = 0;
    let timeout_count = 20;
    loop{
        if let Ok(ret) = r.try_next(){
            println!("{:?}", ret);
        }
        else{
            println!("Err");
        }

        thread::sleep_ms(100);
        i += 1;
        if i >= timeout_count{
            break;
        }
    }
}

fn test20(){
    let a = 10;
    let mut v = vec![10, 1, 4, 200];
    v.sort();
}

// use tokio::prelude::*;
// use tokio::timer;

// fn foo(x: i32) -> Box<Future<Item=(), Error=()> + Send>{
//     if x == 0{
//         Box::new(future::result(Ok(())))
//     }
//     else{
//         Box::new(future::result(Ok(())))
//     }
// }

// fn test21(){
//     tokio::run(foo(12));
// }

fn test21(){
    let (tx, rx) = oneshot::channel();
    let _ = tx.send(100u32);

    let async_1 = async move { 

        if let Ok(ret)= rx.await{
            println!("{}", ret);
            ret
        }
        else{
            println!("{}", 0);
            0
        }
     }.boxed();

    futures::executor::block_on(async_1);

    let (tx, rx) = oneshot::channel();
    let _ = tx.send(100u32);
    let async_2 = async move { 
        if let Ok(ret)= rx.await{
            println!("{}", ret);
            ret
        }
        else{
            println!("{}", 0);
            0
        }
     };

    futures::executor::block_on(async_2);

    //  println!("{:?}", ret);
}

fn main(){
    // futures::executor::block_on(test16());
    test21();
}