#![allow(dead_code)]
#![allow(unused_variables)]

macro_rules! my_eq{
    ($l:expr, $r:expr) => {
        if $l == $r{
            println!("eq: {} == {}", $l, $r);
        }
        else{
            // return false;
            println!("ne: {} != {}", $l, $r);
        }
    };
}

#[cfg(test)]
mod macro_test{

    #[test]
    fn test_01(){
        // println!("ssss");
        let a = 10;
        let b = 20;
        my_eq!(a, b);
        // let ret = my_eq!(a, b);
    }

    #[test]
    fn test_02(){
        let a = 4;
        assert!(a>10, "a is > 10");
    }

    #[test]
    fn test_03(){
        let v = vec![10, 11, 12, 13, 14];

        // for i in v.into_iter(){
        //     println!("{}", i);
        // }

        // let _ = v.into_iter().map(|i|{
        //     println!("{}", i);
        //     i+1
        // }).collect();

        // let ret :Vec<u32> = v.into_iter().map(|x|{
        //         println!("{}", x);
        //         x*2
        //     }).collect();
        // println!("{:?}", ret);

        let v_iter = v.iter();

        for i in v_iter{
            println!("iter: {}", i);
        }
    }

    #[test]
    fn test_04(){
        let x = 100u64;

        let v = x.to_be_bytes();
        println!("{:#?}", v);

        let v2 = vec![1,2,3,4];
        println!("{:#?}", v2);
    }

    #[test]
    fn test_05(){
        let n = 10086u64;
        // let raw = n.to_be_bytes();
        // println!("{:?}", raw);

        // let recover_n = u64::from_be_bytes(raw);
        // println!("{}", recover_n);

        // let recover_n = u64::from_le_bytes(raw);
        // println!("{}", recover_n);

        let raw_vec :Vec<u8> = vec![102, 39, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut raw: [u8;8] = [0u8; 8]; 
        raw[0..8].copy_from_slice(&raw_vec[1..9]);
        println!("{:?}", raw);
        let recover_n = u64::from_le_bytes(raw);
        println!("{}", recover_n);
    }

    #[test]
    fn test_06(){
        // let a = 2;
        // let b = 3;
        // let c = 4;

        // let ret = a^b^c;
        // println!("{}", ret);

        // let ret = b^a^c;
        // println!("{}", ret);

        // let ret = b^c^a;
        // println!("{}", ret);

        // let a = 0;
        // let b = 3290;
        // let c = a^b;
        // println!("{}", c);

        let mut v = vec![10, 12, 13];
        v.push(14);
        println!("{:?}", v);
    }

    #[test]
    fn test_07(){
        let a = 10u64;
        let b = 20u64;
        let c = 30u64;

        println!("{}, {}, {}", a^b, a^c, b^c);
    }

    #[test]
    fn test_08(){
        let v = vec![10, 11, 12, 13, 14];
        let mut v_iter = v.iter();

        // println!("{:?}", v_iter.next());
        // println!("{:?}", v_iter.next());

        if let Some(n) = v_iter.next(){
            println!("{}", n);
        }
    }

    use std::borrow::Cow;
    #[test]
    fn test_09(){

        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        abs_all(&mut input);
        println!("{:?}, {:?}", slice, input);

        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        abs_all(&mut input);
        println!("{:?}, {:?}", slice, input);

        let mut input = Cow::from(vec![-1, 0, 1]);
        abs_all(&mut input);
    }

    fn abs_all(input: &mut Cow<[i32]>){
        for i in 0..input.len(){
            let v = input[i];
            if v < 0 {
                input.to_mut()[i] = -v;
                // input[i] = -v;
            }
        }
    }

    #[test]
    fn test_10(){
        let s = String::from("AB");
        print_addr(&s);

        // let cow1 = Cow::Borrowed(&s);
        // let s1 = cow1.into_owned();
        // print_addr(&s1);

        // let mut cow2 = Cow::Borrowed(&s);
        // cow2.to_mut().insert_str(2, "cd");
        // let s2 = cow2.into_owned();
        // print_addr(&s2);

        let mut s3_1 = s.clone();
        s3_1.insert_str(1, "MM");
        println!("{}, {}", s, s3_1);
        print_addr(&s3_1);

        let mut s3_2 = s.to_owned();
        s3_2.insert_str(1, "MM");
        println!("{}, {}", s, s3_2);
        print_addr(&s3_2);

        // let mut s3_3 = s.to_mut();
        // println!("{}, {}", s, s3_3);
    }

    fn print_addr(s: &str){
        println!("{}", s);

        let mut p = s.as_ptr();
        for ch in s.chars(){
            println!("\t{:p}\t{}", p, ch);
            p = p.wrapping_add(ch.len_utf8());
        }
    }

    #[test]
    fn test_11(){
        for i in 0..10{
            match i%2{
                1 =>{
                    println!("0");
                },
                0 =>{
                    println!("1")
                },
                _ =>{
                    println!("{}", i);
                },
            }
        }
    }
}