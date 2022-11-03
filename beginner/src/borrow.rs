
#[allow(dead_code)]
pub fn main(){
  println!("Hello friend ! ");
  let mut text = String::from("Hello friend ");
  let length = to_uppercase(& mut text);
  println!(" The data : {}  and length {}", text, length);
}

#[allow(dead_code)]
pub fn to_uppercase(text1: &mut String)->  usize{
    println!("The text is : {} ", text1.to_uppercase());
    text1.push_str(" of the day ");
    let length = text1.len();
    return length;
}