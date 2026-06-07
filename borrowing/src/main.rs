// Let’s recap what we’ve discussed about references:
// - At any given time, you can have either one mutable reference or any number of immutable references.
// - References must always be valid.

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    // let mut s1 = &mut s;
    // let mut s2 = &mut s; // can't borrow a mutable ref more than once at a time

    //  change(&s); // results in: some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

    change(&mut s);

    println!("The value of s is '{s}'");

    // no dangling refs
    //    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

//fn dangle() -> &String { // dangle returns a reference to a String
//
//    let s = String::from("hello"); // s is a new String
//
//    &s // we return a reference to the String, s
//} // Here, s goes out of scope and is dropped, so its memory goes away.
// Danger!
