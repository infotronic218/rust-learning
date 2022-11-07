#[allow(dead_code)]
pub fn main(){
    let mut x: u8 = 10 ;
    let mut y = 30 ;
    println!("X is : {} and Y is : {} ", x, y);
    x =  y ;
    y = 30 ;
    let c = (x as f64) * 22.0/7.0 ;
    println!("X is : {} and Y is : {} ", x, y);
    println!(" The value of C : is {:.3} ", c) ;
}