use std::cmp;
// https://play.rust-lang.org/?code=fn%20sum_vec(memo%3A%20%26Vec%3Ci32%3E)-%3Ei32%7B%0A%20%20%20%20let%20mut%20sum%20%3D%200%3B%0A%20%20%20%20for%20i%20in%20memo.iter()%7B%0A%20%20%20%20%20%20%20%20sum%20%3D%20sum%20%2B%20i%3B%0A%20%20%20%20%7D%0A%20%20%20%20sum%0A%7D%0A%0Afn%20foo(v1%3AVec%3Ci32%3E%2C%20v2%3AVec%3Ci32%3E)%7B%0A%7D%0A%0Afn%20main()%20%7B%0A%20%20%20%20let%20v1%20%3D%20vec!%5B1%2C%202%2C%203%5D%3B%0A%20%20%20%20let%20v2%20%3D%20vec!%5B4%2C%205%2C%206%5D%3B%0A%0A%20%20%20%20let%20answer%20%3D%20foo(v1%2C%20v2)%3B%0A%20%20%20%20println!(%22%7B%7D%22%2C%20answer)%3B%20%20%20%0A%7D%0A&version=stable&backtrace=0

fn sum_vec(memo: &Vec<i32>)->i32{
    let mut sum = 0;
    for i in memo.iter(){
        sum = sum + i;
    }
    sum
}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>)->i32{
    let mut v = vec![0; 3];
    let length = cmp::min(v1.len(), v2.len());
    for i in 0..length{
        v[i] = v1[i] + v2[i];
    }
    sum_vec(&v)
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let answer = foo(&v1, &v2);
    println!("{}", answer);
}
