#![allow(unused_imports)]
#![allow(dead_code)]
use futures::future::{self, Either};
use std::thread;
use std::error::Error;
use std::time::{Duration, Instant};
use futures_timer::Delay;
use std::io::{self, Write};
use std::fmt::Debug;
use std::rc::Rc;
use std::ops::Add;

fn loop_value()->u32{
    let mut i = 2;
    loop{
        i += 1;
        if i > 90{
            break (i+10);
        }
    }
}

fn test01(){
    let n = loop_value();
    println!("{:?}", n);
}

fn test02(){
    let x = (1,);
    let y = (1);
    let z = 1;

    if y == z{
        println!("y==z");
    }

    // if x == y {
    //     println!("x == y");
    // }
    println!("{:?}, {:?}, {:?}", x, y, z);
}

async fn test03(){
    let fut_a = future::ready(10);
    let fut_b = future::ready(12);

    let result = match futures::future::select(fut_a, fut_b).await{
        Either::Left(_) => {
            println!("fut_a finished");
            10
        },
        Either::Right(_) => {
            println!("fut_b finished");
            13
        },
    };

    println!("{}", result);
}

async fn test04(){
    let async_a = async {
        10
    };

    let async_b = async {
        23
    };

    let result = future::join(async_a, async_b);
    println!("{:?}",  result.await);
}

async fn test05(){
    // async fn two() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    //     let dur = Duration::from_millis(10);
    //     Delay::new(dur).await;
    //     Delay::new(dur).await;
    //     Ok(())
    // }

    let fut_a = {
        let dur = Duration::from_millis(1000);
        println!("left wait...");
        Delay::new(dur).await;
        println!("left <");
        future::ready("AAA")
    };

    let fut_b = {
        let dur = Duration::from_millis(10);
        println!("right wait");
        Delay::new(dur).await;
        println!("right <");
        future::ready("BBB")
    };

    let result = match futures::future::select(fut_a, fut_b).await{
        Either::Left(v)=>{
            // println!("left");
            v
        },
        Either::Right(v)=>{
            // println!("right");
            v
        },
    };
    println!("{:?}", result);
}

async fn test06(){
    let mut a = future::ready(10);
    // let mut b = future::ready(1000);
    let mut b = future::pending::<u32>();

    let result = match future::select(a, b).await{
        Either::Right(v)=>{
            println!("right");
            return
        },
        Either::Left(v)=>{
            println!("left");
            // return
            v
        },
    };
    println!("{:?}", result);

    // let result = futures::select!{
    //     b_res = b=>{
    //         b_res
    //     },
    //     a_res = a =>{
    //         a_res
    //     },
    // };

    // println!("{}", result);
}

fn test07(){
    let x :Result<u32, &str>= Ok(100u32);
    println!("{:?}", x.ok());

    let x :Result<u32, &str>= Err("AAA");
    println!("{:?}", x.ok());
}

fn test08(){
    let s_vec = vec!["tofu", "120", "119"];
    let numbers :Vec<_>= s_vec.into_iter().map(|s|s.parse::<i32>()).collect();
    println!("{:?}", numbers);

    let s_vec = vec!["tofu", "120", "119"];
    let numbers :Vec<_>= s_vec.into_iter().filter_map(|s|s.parse::<i32>().ok()).collect();
    println!("{:?}", numbers);

    let s_vec :Vec<Result<u32, &str>> = vec![Ok(100u32), Ok(120u32), Ok(111u32)];
    let n_vec: Vec<_> = s_vec.into_iter().filter_map(|s|s.ok()).collect();
    println!("{:?}", n_vec);

    let s_vec :Vec<Result<u32, &str>> = vec![Err("AAA"), Err("BBB"), Err("CCC")];
    let ret  = s_vec.into_iter().filter_map(|s|s.ok()).next();
    println!("{:?}", ret);

    let s_vec :Vec<Result<u32, &str>> = vec![Err("AAA"), Ok(120u32), Ok(111u32)];
    let ret  = s_vec.into_iter().filter_map(|s|s.ok()).next();
    println!("{:?}", ret);
}

fn test09(){
    let s_vec = vec!["tofu", "120", "119"];
    let numbers :Vec<_> = 
        s_vec.into_iter().map(|s|s.parse::<i32>()).collect();
    println!("{:?}", numbers);

    let s_vec = vec!["tofu", "120", "119"];
    let numbers: Result<Vec<_>, _> = 
        s_vec.into_iter().map(|s|s.parse::<i32>()).collect();
    println!("{:?}", numbers);

    let s_vec = vec!["10", "120", "119"];
    let numbers: Result<Vec<_>, _> = 
        s_vec.into_iter().map(|s|s.parse::<i32>()).collect();
    println!("{:?}", numbers);
}

fn test10(){
    trait TraitL1{
        fn fn_in_trait_1(&self){
            println!("fn in trait 1");
        }
    }

    trait TraitL2: TraitL1{
        fn fn_in_trait_2(&self){
            println!("fn in trait 2");
        }
    }

    struct SA{}
    impl TraitL1 for SA{}
    impl TraitL2 for SA{}

    let a = SA{};
    a.fn_in_trait_2();
}

async fn afn1(){
    println!("async fn1 wait...");
    for _ in 0..10{
        print!("1");
        std::io::stdout().flush().unwrap();
        let delay = Delay::new(Duration::new(0, 300_000_000));
        delay.await;
    }
    println!("async fn1 finished");
}

async fn afn2(){
    println!("async fn2 wait...");
    for _ in 0..20{
        print!("2");
        std::io::stdout().flush().unwrap();
        let delay = Delay::new(Duration::new(0, 300_000_000));
        delay.await;
    }
    println!("async fn2 finished");
}

async fn test11(){
    // let delay_2 = Delay::new(Duration::new(1, 0));
    // match futures::future::select(delay_1, delay_2).await{
    //     Either::Left(_)=>{
    //         println!("delay_1 finish");
    //     }
    //     Either::Right(_)=>{
    //         println!("delay_2 finish");
    //     }
    // }
    match futures::future::select(Box::pin(afn1()), Box::pin(afn2())).await{
        Either::Left(_)=>{
            println!("left");
        },
        Either::Right(_)=>{
            println!("right");
        },
    }
}

struct Pos<T1, T2>{
    x: T1,
    y: T2
}

impl<T1, T2> Pos<T1, T2>{
    fn pos_fn(&self){
        println!("pos_fn");
    }
}

impl<T> Pos<T, T>{
    fn same_fn(&self){
        println!("same type");
    }
}

impl<T> Pos<u32, T>{
    fn left_u32_show(&self){
        println!("{}", self.x);
    }
}

impl Pos<u32, u32>{
    fn u32_pos_fn(&self){
        println!("{}, {}", self.x, self.y);
    }
}

trait PosTrait<T, U>{ 
    type Output;
    fn trait_show(&self);
    fn area(&self)->Self::Output;
}

impl<T, U> PosTrait<T, U> for Pos<T, U>
where 
    T: std::fmt::Display + Clone,
    U: std::fmt::Display,
{
    type Output = T;

    fn trait_show(&self){
        println!("{}, {}", self.x, self.y);
    }

    fn area(&self)->Self::Output{
        return self.x.clone();
    }
}

fn test12(){
    // println!("trait test");
    let p1 = Pos{x: 100u32, y: 0.4f32};
    // println!("{}, {}", p1.x, p1.y);
    // p1.pos_fn();
    // p1.left_u32_show();
    p1.trait_show();

    // p1.same_fn();

    let p2 = Pos{x: 40u32, y: 19u32};
    // p2.u32_pos_fn();
    // p2.same_fn();
    // p2.left_u32_show();
    p2.trait_show();

    println!("{}", p2.area());
}

use std::ops::AddAssign;

fn test13(){
    #[derive(Debug)]
    struct Point(i64, i64);

    impl AddAssign for Point{
        fn add_assign(&mut self, rhs: Self){
            self.0 += rhs.0;
            self.1 += rhs.1;
        }
    }

    let mut p1 = Point(12, 23);
    let p2 = Point(100, 100);
    p1 += p2;
    dbg!(p1);
}

fn test14(){
    #[derive(Debug)]
    struct Point<T>(T, T);

    impl<T: AddAssign> AddAssign for Point<T>{
        fn add_assign(&mut self, rhs: Self){
            self.0 += rhs.0;
            self.1 += rhs.1;
        }
    }

    let mut p1 = Point(0.1f32, 0.2f32);
    let p2 = Point(0.4f32, 0.2f32);

    p1 += p2;
    dbg!(p1);
}

fn test15(){
    // #[derive(Debug)]
    struct Point<T1, T2>(T1, T2);

    trait Summary{
        fn summary(&self)->String;
    }

    impl<T1: Debug, T2: Debug> Summary for Point<T1, T2>{
        fn summary(&self)->String{
            format!("summary: ({:?}, {:?})", self.0, self.1)
        }
    }

    impl<T> Summary for Vec<T>{
        fn summary(&self)->String{
            format!("summary len: {}", self.len())
        }
    }

    let p1 = Point(0.1f32, 10u32);
    // println!("{}", p1.summary());

    let v1 = (1..10).collect::<Vec<_>>();
    // dbg!((v1.summary()));

    fn summarize0<T: Summary>(v: &T){
        dbg!(v.summary());
    }
    summarize0(&p1);

    fn summarize1(v: &impl Summary){
        dbg!(v.summary());
    }
    summarize1(&p1);

    fn summarize2(v: &dyn Summary){
        dbg!(v.summary());
    }
    summarize2(&v1);
}

fn test16(){
    struct Point<T>(T, T);

    impl<T> Point<T>
    where T:Copy
    {
        fn left(&self)->T{
            self.0
        }
    }

    let p1 = Point(10u32, 10u32);
    dbg!(p1.left());
}

fn test17(){
    trait TraitA{
        fn mmm(&self){
            println!("TraitA mmm");
        }

        fn fn_a(&self){
            println!("TraitA fn_a");
        }
    }

    trait TraitB{
        fn mmm(&self){
            println!("TraitB mmm");
        }

        fn fn_b(&self){
            println!("TraitB fn_b");
        }
    }

    struct Point(i32, i32);

    impl TraitA for Point{}
    impl TraitB for Point{}

    let p1 = Point(12, 12);
    (&p1 as &dyn TraitA).mmm();
    (&p1 as &dyn TraitB).mmm();

    // (&p1 as &impl TraitB).mmm();

    p1.fn_a();
    p1.fn_b();
}

fn test18(){
    let r1 = Rc::new(10);
    let r2 = Rc::clone(&r1);

    dbg!(r1);
    dbg!(r2);
}

fn test19(){
    let v = (1..20).collect::<Vec<_>>();
    println!("{:?}", v);
}

fn test20(){
    #[derive(Debug)]
    struct Point(i32, i32);

    impl Add for Point{
        type Output = Self;

        fn add(self, rhs: Self)->Self::Output{
            Point(
                self.0 + rhs.0,
                self.1 + rhs.1,
            )
        }
    }

    let p1 = Point(10, 10);
    let p2 = Point(20, 20);

    let p3 = p1 + p2;
    println!("{:?}", p3);
}

fn test21(){
    #[derive(Debug)]
    struct Millis(u32);

    struct Second(u32);

    impl Add<Second> for Millis{
        type Output = Millis;

        fn add(self, rhs: Second)->Self::Output{
            Millis(self.0+rhs.0*1000)
        }
    }

    let s = Second(10);
    let m = Millis(10);

    let r = m + s;
    println!("{:?}", r);
}

fn test22(){
    trait BlockT{
        type Header: std::fmt::Display;
    }

    trait SyncOracle<B: BlockT>{
        fn send_header(&self, h: B::Header){
            println!("send_header, {}", h);
        }
    }

    impl BlockT for u32{
        type Header = u64; 
    }

    impl SyncOracle<u32> for u32{}

    let x = 100u32;
    x.send_header(22u64);
}

fn test23(){
    trait BlockT{
        // type Header: std::fmt::Display;
    }

    // trait SyncOracle<B: BlockT>
    trait SyncOrcal
    where Self: std::fmt::Display
    {
        // fn send_header(&self, h: B::Header){
        //     println!("send_header, {}", h);
        // }
        fn send_header(&self){
            println!("{}", &self);
        }
    }

    impl SyncOrcal for u32{};

    let x = 100u32;
    x.send_header();
}

fn test24(){

    // struct BTCBlock{
    //     header: u32,
    //     body: Vec<u8>,
    // }

    // impl Block for BTCBlock{
    //     type Header = Self::header;
    //     type Body = Self::body;

    //     fn header(&self)->Self::Header{
    //         self.header
    //     }

    //     fn body(&self)->Self::Body{
    //         self.body
    //     }
    // }

    trait Block{
        type Header;
        type Body;

        fn header(&self)->&Self::Header;
        fn body(&self)->&Self::Body;
    }

    struct GenBlock<H,B>{
        header: H,
        body: B
    }

    impl<H,B> Block for GenBlock<H,B>{
        type Header = H;
        type Body = B;

        fn header(&self)->&Self::Header{
            &self.header
        }
        fn body(&self)->&Self::Body{
            &self.body
        }
    }

    let btc_block = GenBlock{header: 100u32, body: vec![10, 10, 10]};
    println!("{}, {:?}", btc_block.header(), btc_block.body());

    let eth_block = GenBlock{header: "E122", body: vec!["A", "B", "C"]};
    println!("{}, {:?}", eth_block.header(), eth_block.body());
}

fn test25(){
    struct Point<T>(T, T);

    let p1 = Point(10, 10);
}

fn main(){
    test24();
    // futures::executor::block_on(test11());
}