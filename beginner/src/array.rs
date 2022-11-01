pub fn main(){
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x' ;

    // Integer array definition 
    let numbers: [i32; 5] ;
    numbers = [0; 5] ;

    let index: usize = numbers.len();
    println!("The last number is : {}", numbers[index-1]) ;

    // Two dimensions arrays 
    let tab = [[1, 2, 3], [5, 6, 4] ];
    let data   = tab[1][1] ;
    println!("The data is : {}", data);

    // Tupples in the data can save multiple type of element
    let mut things = ( 10, 3.45, 'x');
    println!("The tuple data content 0 : {} ", things.0);
    things.0 += 10 ;
    println!("The tuple data content 0 : {} ", things.0);
    println!("The tuple data content 1 : {} ", things.1);

    // Unpack the tuple content 
    let (a, b, c) = things ;
}