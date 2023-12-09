pub fn main() {
  // String Literal Type: immutable
  let s = "hello";
  // s.push_str(", world!"); 
  println!("{}", s);

  // String Type: mutable
  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("{}", s);
}