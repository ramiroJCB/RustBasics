fn main() {
    const  X :i32= 5;
    let n_of_houses:i32 =clc_houses(X);
    let mut  y=X;
    y+=1;

    println!("The value of X is: {}", X+y);
    println!("The value of X is: {} ,{}", y,X);
    println!("The number of houses is :{}",n_of_houses);
    shadow_variables(n_of_houses);
    shadowing_vs_reassing();
}

 fn clc_houses(x:i32)->i32{
     return x+5
 }

 fn shadow_variables(number:i32){
     let shadow =number;
     let shadow=shadow+1;
     let shadow =shadow*2;
     
     println!("you can actually do this shadows:{}",shadow)

 }

 fn shadowing_vs_reassing(){
     let lens ="  ";
     let lens =lens.len();
     println!("should be 2:{}",lens);

    //But if  try  to reassing a variable
    // let mut lens ="  ";
      
     //lens =lens.len();
     //You will get  a  compile error because 
     //we cannot change the initial value type 
    
 }