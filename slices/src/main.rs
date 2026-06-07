fn main() {
  let s = String::from("hello world");
  let s1 = "goodbye world";
  let word = first_word(&s);
  let sword = first_word(s1);

  println!("{word}");
  println!("{sword}");
   
  let hello = &s[..5];
  let world = &s[6..];
  println!("{hello}");
  println!("{world}");
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }
  
  &s[..]
}

// not efficient or reusable
//fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes();
//
//   for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return i;
//        }
//    }
//
//    s.len()
//}
