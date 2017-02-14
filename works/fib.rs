#![allow(dead_code)]
// https://play.rust-lang.org/?code=%23!%5Ballow(dead_code)%5D%0A%0Afn%20fib(n%3Au32)%20-%3E%20u32%7B%0A%7D%0A%0A%0A%23%5Btest%5D%0Afn%20test_base_case()%7B%0A%20%20%20%20assert_eq!(fib(0)%2C%200)%3B%0A%20%20%20%20assert_eq!(fib(1)%2C%201)%3B%0A%7D%0A%0A%23%5Btest%5D%0Afn%20test_induction_case()%7B%0A%20%20%20%20assert_eq!(fib(2)%2C%201)%3B%0A%20%20%20%20assert_eq!(fib(26)%2C%20121393)%3B%0A%20%20%20%20assert_eq!(fib(38)%2C%2039088169)%3B%0A%7D%20%20%20%20&version=stable&backtrace=0

fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 2) + fib(n - 1)
    }
}

#[test]
fn test_base_case(){
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
}

#[test]
fn test_induction_case(){
    assert_eq!(fib(2), 1);
    assert_eq!(fib(26), 121393);
    assert_eq!(fib(38), 39088169);
}
