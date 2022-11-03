#[allow(dead_code)]
use crate::factorial;
pub fn main(){
    println!("The ownership dans le temps ");
    let testing =  10 ;
    println!("The factorial of {} is : {} ", testing, factorial::fact2(testing));
    println!("The value is : {} ", testing);

    let mut text = String::from("Burkina ");
    println!("The country is : {}", text);
    text.push_str("Faso");
    println!("The country is : {}", text);

    // Only on owner of the data 
    let outer_planet : String ;

    {
        let mut inner_planet  = String::from(" Mercury");
        println!("Inner planet is {} ", inner_planet);
        outer_planet = inner_planet.clone() ;
        inner_planet.push_str(" I changed ");
        println!("The Inner instance is : {} ", inner_planet);
    }
    println!("The outer planet is : {} ", outer_planet);

 
}


