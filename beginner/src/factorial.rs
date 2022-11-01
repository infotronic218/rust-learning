pub fn fact(num :i32) -> i32{
    if(num ==0){
        return 1 ;
    }else if(num==1){
        return 1 ;
    }
    return num *fact(num-1);
}

pub fn fact2(num :i32)->i32{
    let mut i = 0 ;
    let mut f : i32 = 1 ;
    while(i<=num){
        i += 1 ;
        f = f*i ;
    }
    return f ;
}

pub fn main(){
    println!("The factorial of 0  is : {}", fact2(0));
    println!("The factorial of 1  is : {}", fact2(1));
    println!("The factorial of 3  is : {}", fact2(3));
    println!("The factorial of 5  is : {}", fact2(5));
}