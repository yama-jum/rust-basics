pub fn main() {
  // String Literal Type: immutable
  let s = "hello";
  // s.push_str(", world!");
  println!("{}", s);

  // String Type: mutable
  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("{}", s);

  /*
   * NOTE: s1をs2に代入すると、String型のデータがコピーされる。
   * つまり、スタックにあるポインタ、長さ、 許容量をコピーするということで、ポインタが指すヒープ上のデータはコピーしない
   * -> jsのオブジェクトの参照渡しと同じ?? ただsiはs2にmoveされるので、s1は無効になる
   */
  let s1 = String::from("hello");
  let _s2 = s1;
  // println!("{}, world!", s1); // error: value borrowed here after move

  /*
   * NOTE: clone()を使うと、ヒープ上のデータもコピーされる
   */
  let s3 = String::from("hello");
  let s4 = s3.clone();
  println!("s3 = {}, s4 = {}", s3, s4);
}
