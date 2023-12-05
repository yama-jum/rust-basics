pub fn main() {
  // Parameters
  another_function(5);
  print_labeled_measurement(5, 'h');

  // Statements and Expressions
  let y = {
    let x = 3;
    x + 1
  };
  println!("The value of y is: {}", y);

  // Functions with Return Values
  let z = five();
  println!("The value of z is: {}", z);

  let a = plus_one(5);
  println!("The value of a is: {}", a);
}

fn another_function(x: i32) {
  println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  // NOTE: `x + 1` is an expression but `x + 1;` is a statement
  x + 1
}