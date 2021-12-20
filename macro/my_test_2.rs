macro_rules! assert_eq {
    (l:$expr, r:$expr) => {
        if l==r{
            return true;
        }
        else{
            return false;
        }
    };
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_print(){
        println!("ssss");
    }
}
pub fn add(a: i32, b: i32) -> i32{
    return a + b;
}

pub fn bad_add(a: i32, b: i32)->i32{
    return a+b+1;
}

// // 这个最简单的版本好像也可以使用，结果也符合预期
// macro_rules! my_assert_eq {
//     ($l:expr, $r:expr) => {
//         if ($l== $r){
//             println!("eq2");
//         }
//         else{
//             println!("ne");
//         }
//     };
// }

// #[cfg(test)]
// mod test2{
//     use super::*;

//     #[test]
//     fn test_marco2(){
//         let a = 10;
//         let b = 5+5+2;
//         my_assert_eq!(a, b);

//         assert_eq!(a, b, "cmp {} and {}", a, b);
//     }

//     #[test]
//     fn test_add2(){
//         let n = add(1,2);
//         println!("n: {}\n", n);
//         assert_eq!(n, 3);
//         assert_eq!(n, 3,);
//     }

//     #[test]
//     fn test_bad_add2(){
//         let n = bad_add(1,2);
//         println!("n: {}\n", n)
//     }
// }
