
#[allow(dead_code)]
pub fn main(){
    println!("Speaking of life time");
    //let result   ;
    let p1 = String::from("P251");
    let p2 = String::from("P2");
    let mut result = best_fuel(&p1, &p2);
    println!("The results is : {}", result);
    result = best_fuel2(&p1, &p2);

}

fn best_fuel<'a>(x: &'a str, y:&'a str)->&'a str{
    if x.len()< y.len(){
        return y ;
    }else{
        return x ;
    }
}
fn best_fuel2<'a>(x: &'a str, y:&'a str)->&'a str{
    if x.len()< y.len(){
        return y ;
    }else{
        return x ;
    }
}
