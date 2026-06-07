fn main() {

    // let s = "hello"; // string literal immutable

    let mut st = String::from("hello");
    st.push_str(", world!"); // appends literal to a String type
    println!("{st}");

    // this will cause an error as s1 is now an invalid reference
    // this is a move, not a shallow copy, s1 dropped as out of scope
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}, world!");

    // nothing referring to original s so it is dropped as out of scope
    let mut s = String::from("hello");
    s = String::from("ahoy");
    println!("{s}, world!");    

    // this is fine explicit copy
    let s3 = String::from("hi");
    let s4 = s3.clone();
    println!("s3 = {s3}, s4 = {s4}");

    // no clone as on stack so no difference from deep or shallow copy
    let x = 5;
    let _y = x;

    // passing args to a function will move ownership unless it has Copy
    // thus we use References like this

    let s5 = String::from("sup");
    let len = calculate_length(&s5);
    println!("len of {s5} is {len}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
