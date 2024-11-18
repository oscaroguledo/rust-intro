fn main() {
    let company:&str="TutorialsPoint";
    let location:&str = "Hyderabad";
    println!("company is : {} location :{}",company,location);
    let empty_string = String::new();
    println!("length is {}",empty_string.len());

    let content_string = String::from("TutorialsPoint");
    println!("length is {}",content_string.len());
    let name1 = "Hello TutorialsPoint , 
   Hello!".to_string();         //String object
   let name2 = name1.replace("Hello","Howdy");    //find and replace
   println!("{}",name2);
   let msg = "Tutorials Point has good t
   utorials".to_string();
   let mut i = 1;
   
   for token in msg.split_whitespace(){
      println!("token {} {}",i,token);
      i+=1;
   }
 }