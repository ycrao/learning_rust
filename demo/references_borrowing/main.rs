fn foo(_v1: &Vec<i32>, _v2: &Vec<i32>) -> i32 {
    42
}


fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let answer = foo(&v1, &v2);
    println!("v1 : {:?}, v2: {:?}, answer: {}", v1, v2, answer);
    for i in v1 {
        println!("A reference to {}", i);
    }

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }

    println!("{}", x);

}
