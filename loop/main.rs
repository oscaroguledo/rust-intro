fn main(){
    for x in 1..11{ // 11 is not inclusive
       if x==5 {
          continue;
       }
       println!("x is {}",x);
    }
    let mut x = 0;
    while x < 10{
        x+=1;
        println!("inside loop x value is {}",x);
    }
    println!("outside loop x value is {}",x);
    //while true

   let mut x = 0;
   loop {
      x+=1;
      println!("x={}",x);

      if x==15 {
         break;
      }
   }
 }