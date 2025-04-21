const DEFAULT_X_VALUE: u32 = 5;

fn main() {
    let x = DEFAULT_X_VALUE;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
