pub fn main(){
  println!("Binary testing ");
  let mut x = 0b1001_1001u8;
  println!("The value of X is : {}", x);
  println!("The value of X is : {:08b}", x);
  x =  !x ;
  println!("The inverted value : {:08b}", x) ;
  // Clear the bit at a postion 
  x &= !(0b0000_0010u8);
  println!("The inverted value : {:08b}", x) ;
}