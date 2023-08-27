fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;
    let x = x + 1;

    println!("The value of constant is: {THREE_HOURS_IN_SECONDS}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}
