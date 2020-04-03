#[derive(Debug,Clone)]
struct MyStruct {
    val1: i32,
    val2: u8,
    val3:String,
    val4:HttpEnum
}
fn create_mystruct(val1: i32,val2:u8,val3:String,val4:HttpEnum) -> MyStruct {
    MyStruct{
        val1,
        val2,
        val3,
        val4
    }
}

impl MyStruct {
   fn show(&self) {
      println!("this struct val1 is : {} and val2 is : {}",self.val1,self.val2);
   }

   fn change_val1(&mut self,val1:i32){
      self.val1=val1 
   }
}
#[derive(Debug,Clone)]
enum HttpEnum {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
    
}
#[derive(Debug)]
enum SecondEnum {
    Variant1(String),
    Variant2{int:u128},
}
pub fn run() {
    let my_first_struct=MyStruct{
        val1:-1,
        val2:1,
        val3:String::from("firs struct"),
        val4:HttpEnum::Ok
    };
    let mut my_second_struct=MyStruct{val1:-2,..my_first_struct.clone()};
    println!("{:?}",my_first_struct);
    println!("{:?}",my_second_struct);
    my_second_struct=create_mystruct(-2, 2,String::from("second struct"),HttpEnum::InternalServerError);
    println!("{:?}",my_second_struct);
    my_second_struct.show();
    MyStruct::show(&my_second_struct);
    my_second_struct.change_val1(-85);
    my_second_struct.show();
    println!("\n\n\n");
    let mut my_enum=HttpEnum::InternalServerError;
    let my_enum = SecondEnum::Variant2{int:85};
    println!("{:?}",my_enum);

}