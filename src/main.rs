fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    takes_ownership(s);

    // simple stack values so both are 5
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
    makes_copy(x); // i32 has copy trait so ok to reuse after

    // s1 is string with 3 parts, pts, len, capacity on the stack
    // the pointer points to actual value on heap i.e. hello
    // side note length is how much memory
    // capacity is total amount of memory in bytes that String recieved from allocator
    let s1 = String::from("Hello");
    // just copied to pointer, length and capacity. did not copy the hello
    let s2 = s1;

    // when they go out of scope they will both try to free same memory
    // leads to memory corruption if allowed
    // rust considered s1 as no longer valid

    println!("{}, world!", s2);

    // not a shallow or deep copy because Rust invalidated s1 so it's a move
    // clone() will deep copy

    let s3 = gives_ownership();
    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);

    let s6 = String::from("yello");
    let len = calculate_length(&s6);
    println!("The length is {len}.");

    let mut s7 = String::from("ello");
    change(&mut s7); // can only have 1 mutable reference at a time
    // prevent data races. two or more pointers accessing same data.
    // at least one of thje pointers is being used to write to the data
    // theres no mechanism being used to sync access to the data
    // can use curly braces to make new scope
    {
        let r1 = &mut s7;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s7;

    // Cannot have mutable references while having immutable ones
    // compiler combats dangling references.
    //
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // word will get the value 5

    // this empties the String, making it equal to ""
    println!("the first word is: {word}");

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
    //
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // value is drop here and memory is free

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len() // len() returns the length of a String
} // Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.

fn change(some_string: &mut String) {
    // mutable reference
    some_string.push_str(", world");
}

//In idiomatic Rust, functions do not take ownership of their arguments unless they need to, and the reasons for that will become clear as we keep going.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
