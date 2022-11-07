use std::env ;

#[allow(dead_code)]
pub fn main(){
 if env::args().len()<= 2{
    println!("Error : the program requires more than 2 argument");
    return;

 }
 println!("Rending environment files");
 for(index, arg) in env::args().enumerate(){
    println!("Arg : {} is {} ", index, arg);
 }

 // reading argument at index
 let arg2 = env::args().nth(2).unwrap();
 println!("Arg 2 is {} ", arg2);
}