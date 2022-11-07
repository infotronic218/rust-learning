#[allow(dead_code)]

struct Person {
    name: String,
    height:f64,
    age:u8 
}

impl Person{
    fn get_name(&self)->&str{
        return &self.name ;
    }
    fn set_age(&mut self, age:u8){
        self.age = age ;
    }
    // create new function 
    fn new(name:&str, age:u8, height:f64)->Person{
      return Person{
        name:String::from(name),
        age:age,
        height:height
      };
    }
}

// Generic dataype
struct Rectangle<T>{
    width:T,
    height:T
}

pub fn main(){
  let mut p1 = Person{
    name:String::from("Ousseni"),
    height:1.85,
    age:26
  };

  let mut p2 = Person{
    name:String::from("Salma"),
    ..p1
  };

  println!("The person name is : {} , age : {} , Heigt: {}", p1.name, p1.age, p1.height);
  p1.age = 27 ;
  println!("The person name is : {} , age : {} , Heigt: {}", p1.name, p1.age, p1.height);
  p2.set_age(25);
  println!("The person name is : {} , age : {} , Heigt: {}", p2.get_name(), p2.age, p2.height);
  //Creating persion 
  let p3 = Person::new("File", 22, 1.65);
  println!("The person name is : {} , age : {} , Heigt: {}", p3.get_name(), p3.age, p3.height);

  // Create generic datatype
  let rect = Rectangle{
    width:8u8,
    height:10u8
  };

  println!("The data content is :W= {} ,H ={}", rect.width, rect.height);
}