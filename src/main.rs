
fn _sum() {
    let a: i32 = 34;
    let b: i32 = 11;

    let mut sum: i32 = add(a, b);
    sum += 1;


    println!("{} + {} = {}", a, b, sum);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn loopy() {
    let values = [8, 13, 99, 2];
    let mut sum = 0;
    for n in 0..4 {
        sum = add(sum, values[n]);
    }

    println!("sum = {}", sum);
}

fn main() {
    loopy()
}
