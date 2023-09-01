fn main() {
    // Integer Types
    let _num = 10; // i32
    let _num2: u32 = 15; // u32

    // Floating-Point Types
    let _f1 = 2.0; // f64
    let _f2: f32 = 3.0; // f32

    // Numeric Operations
    let _sum = 5 + 10; // addition
    let _difference = 95.5 - 4.3; // subtraction
    let _product = 4 * 30; // multiplication
    let _quotient = 56.7 / 32.2; // division
    let _truncated = 43 / 5; // truncated division result is 8
    let _remainder = 43 % 5; // remainder

    // Boolean Type
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Character Type
    // char literals are specified with single quotes, as opposed to string literals, which use double quotes.
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻'; // emoji

    // Compound Types
    // Tuple Type
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = _tup; // destructuring
    println!("The value of y is: {_y}");
    let _five_hundred = _tup.0;
    let _six_point_four = _tup.1;
    let _one = _tup.2;

    // Array Type
    // Unlike a tuple, every element of an array must have the same type.
    // Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.
    // In Rust, the values going into an array are written as a comma-separated list inside square brackets
    // Rust will panic at runtime if you try to access an element using indexing that is outside the bounds of the array.
    // The length of an array is part of its type, so arrays are useful when you want your data allocated on the stack rather than the heap.
    // By default, arrays in Rust are immutable.
    // You’re allowed to change the values of a variable bound to an array, but you can’t change the types of the elements in the array.
    // If you want to create an array that can grow in size, look to the vector type next.

    let _a = [1, 2, 3, 4, 5];
    let _first = _a[0];
    let _second = _a[1];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    let _c = [3; 5]; // [3, 3, 3, 3, 3]
}
