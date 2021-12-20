extern crate futures;
use std::thread;
use std::time::{Duration, SystemTime};
use futures::executor;
use futures::channel::mpsc;
use futures::executor::ThreadPool;
use futures::StreamExt;


async fn async_f1(){
    println!("1111111");
}

async fn async_f2(){
    println!("22222");
}

async fn async_main(){
    let f1 = async_f1();
    let f2 = async_f2();

    let f = async move {
        f1.await;
        f2.await;
    };

    f.await;
}

async fn async_main2(){
    async fn async_f3()->i32{12}

    let func2 = async move {
        let t = 1;
        let v = t+1;
        let b = async_f3().await;
        let rv = &v;

        let result = *rv + b;
        println!("{}", std::mem::size_of_val(&result));
        result
    };

    let fut = func2;
    println!("{}", std::mem::size_of_val(&fut));

    // let fut = func2.await;
    // println!("{}", std::mem::size_of_val(&fut));
}

fn test02(){
    let pool_executor = ThreadPool::new().expect("pool failed");
    let (tx, rx) = mpsc::unbounded::<String>();

    let s = SystemTime::now();

    println!("start");
    let future_values = async {
        let future_tx_result = async move{
            let hello = String::from("Hello world");
            for c in hello.chars(){
                let x = c.clone();
                let tx = tx.clone();
                thread::spawn(move ||{
                    thread::sleep(Duration::from_millis(1000));
                    tx.unbounded_send(x.to_string()).expect("unbound send failed");

                });
            }
        };

        pool_executor.spawn_ok(future_tx_result);
        let future_value = rx.map(|v|v).collect();
        future_value.await
    };

    let values: Vec<String> = executor::block_on(future_values);
    let elapsed = s.elapsed();
    println!("{:?} ({:?})", values, elapsed);
}

fn test03(){
    let x = async {
        let w = String::from("hell");
        w
    };

    let result = executor::block_on(x);
    println!("{}", result);
}

async fn async_test04(){
    let (tx, mut rx) = mpsc::channel::<i32>(1000);

    for i in 0..10{
        let mut tx = tx.clone();
        thread::spawn(move||{
            tx.try_send(i).unwrap();
        });
    }

    for _ in 0..10{
        if let Some(r) = rx.next().await{
            println!("{}", r);
        }
    }
}

fn main(){
    // executor::block_on(func2);
    // println!("{}", std::mem::size_of_val(&fut));
    executor::block_on(async_test04());
    // test02();
}