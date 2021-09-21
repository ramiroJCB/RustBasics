fn main() {
    //Exist Two subsets of types in rusts: 
    //Scalar && compound 
    //being rust a statically type language , the compiler need  everything 
    //typed  at compile time , there  are  only  a few   implicit types 
    //that you can do.

    // we need  a type annotation  whenever  we  try yo  convert types in rust like :

let string :i32= "42".parse().expect("not a number");

 println!("this is  the guess :{}",string);


 scalar_types()
}

fn scalar_types(){
    //Scalar types  are single Value Data .
    //Rust  is complemented by  Four primary scalar types :
    //Integer,floating-point numbers ,Booleans and Characters ?

   //Interges are  types  are prefixed  "u" or "i" standing for  "Unsigned" and "signed"
   // key  differents between those is that "i" can  have  negative values  and "u" cannot 

   let  number:i32=1_000;

   println!("{}",number);

   //Rust come with only two types of  Floeting-points numbers which are just f64(implicit) f32 (explicit)
   let reminder=43%3;
   println!("{}",reminder);

   //Booleans on rust are  just   true / false  

   let bol=true;
   println!("{}",bol);

   

}