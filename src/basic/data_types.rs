pub fn main() {
  /*
   * Scalar Types
   *
   * integer, floating-Point, boolean, character
   */
  // addition
  let sum = 5 + 10;
  println!("sum: {}", sum);
  // subtraction
  let difference = 95.5 - 4.3;
  println!("difference: {}", difference);
  // multiplication
  let product = 4 * 30;
  println!("product: {}", product);
  // division
  let quotient = 56.7 / 32.2;
  let floored = 2 / 3;
  println!("quotient: {}", quotient);
  println!("floored: {}", floored);
  // remainder
  let remainder = 43 % 5;
  println!("remainder: {}", remainder);

  let t = true;
  let f: bool = false;
  println!("t: {}", t);
  println!("f: {}", f);

  let c = 'z';
  let z: char = 'ℤ';
  let heart_eyed_cat = '😻';
  println!("c: {}", c);
  println!("z: {}", z);
  println!("heart_eyed_cat: {}", heart_eyed_cat);

  /*
   * Compound Types
   *
   * tuple, array
   */
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
  println!("The value of z is: {}", z);

  let five_hundred = tup.0;
  let six_point_four = tup.1;
  let one = tup.2;
  println!("five_hundred: {}", five_hundred);
  println!("six_point_four: {}", six_point_four);
  println!("one: {}", one);

  let array_a = [1, 2, 3, 4, 5];
  let months = [
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
  let array_b: [i32; 5] = [1, 2, 3, 4, 5];
  let array_c = [3; 5];
  let first_element = array_a[0];
  println!("array_a: {:?}", array_a);
  println!("months: {:?}", months);
  println!("array_b: {:?}", array_b);
  println!("array_c: {:?}", array_c);
  println!("first_element: {}", first_element);
  // println!("array_a[10]: {}", array_a[10]);  // panic!!: index out of bounds: the len is 5 but the index is 10
}
