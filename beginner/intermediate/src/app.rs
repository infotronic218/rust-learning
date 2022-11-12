#[allow(dead_code)]
pub fn main(){
    println!("Resistor implementation ");
    let vr:f32 = 0.8 ;
    let r1:f32 = 150.0e3 ;
    
    let r2:f32 = 50.0e3 ;
    let r3:f32 = 75.0e3 ;
    let vd:f32 = 3.0 ;
    let vo:f32 = vr *(1.0+ r1/r2) + (r1/r3)*(vr- vd);
    println!("R1 {} ; R2 {} ; R3 : {} ; Vr : {} ; Vd : {}  ", r1,r2,r3, vr, vd);
    println!("The value of Vo is : {} ", vo) ;
}