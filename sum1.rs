// Here is an idiomatic, pro-level way of doing the sum.
fn main() {
    let sum: i32 = (0..5).sum();
    println!("sum is {}\n", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum is {}\n", sum);
}

/* Note that this is one of those cases where you need to be explicit about the type of the variable, 
   since otherwise Rust doesn't have enough information. Here we do sums with two different integer 
   sizes, no problem. (It is also no problem to create a new variable of the same name if you run out of 
    ames to give things.)
*/