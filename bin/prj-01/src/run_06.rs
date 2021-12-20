use std::collections::BTreeMap;
use codec::{Encode, Decode};

fn test01(){
    let foo = 'a';
    // let all_parrten = 'A'..='Z';

    // if matches!(foo, 'A'..='Z'){
    //     println!("yes");
    // }
    // else{
    //     println!("no");
    // }
    dbg!(matches!('m', 'a'..='z'));

    if matches!(foo, 'a'|'b'|'c'){
        println!("yes")
    }
    else{
        println!("no")
    }

    let x = 100u32;

    if x == 100u32{
        println!("yes");
    }
    // let foo2 = Some(10);
}

fn test02(){
    let foo = 'f';
    let r1 = matches!(foo, 'a'..='z');
    dbg!(r1);

    let r2 = match foo{
        'a'..='z'=>true,
        _=>false
    };
    dbg!(r2);
}

fn test03(){
    struct Service;

    impl Service{
        fn service_inner_fn(&self){
            println!("service_inner_fn");
        }
    }

    trait ServiceTrait{
        fn service_trait_fn(&self){}
    }

    // impl<T> ServiceTrait for Arc<T>{
    //     fn service_trait_fn(&self){
    //         <&T>::service_trait_fn(&**self)
    //     }
    // }
}

fn test04(){
    let v = vec![1,2,3,4,5,6];

    // v.iter().for_each(|i|println!("{}", i));
    v.iter().enumerate().for_each(|(i,d)|println!("{}, {}", i,d));
}

fn test05(){
    let x = [(10, "AAA"), (2, "BBB"), (3, "CCC"), (4, "DDD")];
    let mut btmap = BTreeMap::new();
    x.iter().for_each(|i|{ btmap.insert(i.0, i.1); });
    // x.iter().for_each(|i|btmap.insert(i.0, i.1););

    // btmap.iter().for_each(|(k,v)|{println!("{}, {}", k, v);});
    let ret = btmap.iter().map(|(k,v)|v).collect::<Vec<_>>();
    println!("{:?}",ret);

    let ret = btmap.iter().map(|i|i.1).collect::<Vec<_>>();
    println!("{:?}",ret);
}

fn test06(){
    let p1 = vec![1];
    let p2 = vec![2,2];
    let p3 = vec![3,3,3];
    let p4 = vec![4,4,4,4];

    let vec_p = vec![p1, p2, p3, p4];
    let encode_data = vec_p.encode();
    println!("{}", encode_data.len());

    let decode_vec = <Vec<Vec<u32>> as Decode>::decode(&mut encode_data.as_ref());
    if let Ok(d_vec) = decode_vec{
        d_vec.iter().for_each(|i|println!("{:?}", i));
    }
}

fn test07(){
    #[derive(Decode, Encode, Debug)]
    struct VoteData{
        vote_num: u64,
        sync_num: u64,
    }

    // type PeerId = Vec<u8>;

    #[derive(Debug, Encode, Decode)]
    enum MultiType{
        VoteData(VoteData),
        PeerId(Vec<u8>),
    }

    let m1 = MultiType::VoteData(VoteData{vote_num:10, sync_num: 20});
    let m2 = MultiType::PeerId((1..20).collect::<Vec<_>>());

    let m1_encode = m1.encode();
    let m2_encode = m2.encode();

    if let Ok(decode_m1) = <MultiType as Decode>::decode(&mut m1_encode.as_ref()){
        // if let MultiType::VoteData(v) = decode_m1{
        //     println!("m1 correct: {:?}", v);
        // }
        // else{
        //     println!("m1 decode err");
        // }
    }

    if let Ok(decode_m2) = <MultiType as Decode>::decode(&mut m2_encode.as_ref()){
        if let MultiType::PeerId(v) = decode_m2{
            println!("m2 corrent: {:?}", v);
        }
        else{
            println!("m2 decode err");
        }
    }
}

fn test08(){
    // let strings = vec!["tofu", "93", "18"];
    // let numbers: Vec<_> = strings
    //     .into_iter()
    //     .filter_map(|s| s.parse::<i32>().ok())
    //     .collect();
    // println!("Results: {:?}", numbers);

    // let r1 = "tofu".parse::<i32>();
    // println!("{:?}", r1.ok());

    // let r1 = "123".parse::<i32>();
    // println!("{:?}", r1.ok());

    let v= vec![100, 120, 150, 180, 200];
    println!("{:?}", v);
    // let opt_v = v.iter().for_each(|v|Some(v))
    let opt_v = v.iter().map(|v|Some(v)).collect::<Vec<_>>();
    println!("{:?}", opt_v);

    let v2 = opt_v.iter().filter_map(|v|v.as_ref()).collect::<Vec<_>>();
    println!("{:?}", v2);
}

fn test09(){
    let d = [(1, "AAA"), (2, "BBB"), (3, "CCC"), (4, "DDD")];
    let mut bt_map = BTreeMap::new();
    d.iter().for_each(|i|{bt_map.insert(i.0, i.1);});

    let pos = bt_map.iter().position(|(&k,v)|k==3);
    println!("{:?}", pos);
}

fn test10(){
    let v = vec![Some(0), Some(0), Some(1), Some(2), Some(0)];
    let r = v.iter().filter_map(|v|v.as_ref()).fold(0, |r, &x| if x == 0 {r+1} else {r});
    println!("{}", r);
}

fn main(){
    test10();
}