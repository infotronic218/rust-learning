use std::fs ;
use std::io::prelude::*;


#[allow(dead_code)]
pub fn main(){
 println!("Implementing files reading ");
 let contents = fs::read_to_string("./src/names.txt").unwrap();
 println!("The content is {} ", contents);
 // Read by lines


 for (index, line) in contents.lines().enumerate(){
    println!("{} - {} ", index, line);
 }

 // Read text as bytes
 let contents = fs::read("./src/names.txt").unwrap();
 println!("Contents bytes is : {:?}",contents);
 // Writting factorial data 
 let mut file = fs::OpenOptions::new().append(true).open("./src/names.txt").unwrap();  
  for num in 0..10 {
    let fnun = fact(num);
    let mut strs = String::from("\n");
    strs.push_str(&num.to_string());
    strs.push_str(" = ");
    strs.push_str(&fnun.to_string());
    file.write(strs.as_bytes());
    println!(" {} = {} ", num, fnun);
  }
}

#[allow(dead_code)]
pub fn fact(num :i32) -> i32{
    if num ==0 {
        return 1 ;
    }else if num==1 {
        return 1 ;
    }
    return num *fact(num-1);
}