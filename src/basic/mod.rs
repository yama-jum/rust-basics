mod data_types;
mod functions;
mod variables;

pub fn main() {
  println!("## variables");
  variables::main();
  println!("## data_types");
  data_types::main();
  println!("## functions");
  functions::main()
}
