fn check(x: Vec<i32>) -> Option<i32> {
    if x.len() > 6 {
        Some(x[4])
    } else {
        None
    }
}
fn main() {
    match check(vec![1, 2, 3, 4, 5]) {
        Some(num) => {
            println!("the numbr {}", num);
        },

        None => println!("error"),
    }
}
