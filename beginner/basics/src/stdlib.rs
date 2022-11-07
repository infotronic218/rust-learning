

use std::io ;

#[allow(dead_code)]
pub fn main(){
  let mut buffer = String::new();
  println!("Enter a message : ");
  let _ret = io::stdin().read_line(&mut buffer);

  println!("The message is : {}", buffer);
  
  println!("Enter an interger ");
  let mut buffer = String::new();
  let _ret = io::stdin().read_line(&mut buffer);
  let mut number = buffer.trim().parse::<i32>().unwrap();
  number += 1 ;
  println!("The value read is : {}" , number+1);

  /// Using the crates external dependencies
  let _num1 = rand::random::<f64>();
  let _num2 = rand::random::<f64>();
  println!("The numbers are : {}  {} ", _num1, _num2);

}