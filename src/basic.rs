const GLOBAL_CONSTANT: &str = "این یک مقدار Global است و همه‌جا می‌توان به آن دسترسی داشت";

pub fn run() {
    first_section();

    let mut a = String::from("hello");
    let reference1 = &a;
    let reference2 = &a;
    // let  reference3 = &mut a;

    ali(reference1);
    hossein(reference2);
    mohammad(&mut a);
    

    println!("a in main function: {}", a);
}

fn first_section()  {
    let mut x = 10;
    const BUFFER_SIZE: i32 = 10;
    x = 5;
    println!("{}  {} ",x,BUFFER_SIZE);
    let mut a: [i8; 3] = [0, 0, 0];
    let b=[1,2,3];
    a[2]=84;
    println!("{:?}  {:?} ",a,b);
    let tup = (1, true, "hello", 9.99);
    println!("{}", tup.2);
    let a = if false {
        'f'
    }
    else{
        't'
    };
    println!("a is :{} ", a);
    for counter in 2..11 {
        println!("counter = {}", counter);
    }
 



}
fn drow() {
    for i in 1..20 {
       if i<10 {
           for j in 1..i {
              print!("*");
           }
           print!("\n");
       }else {
        for j in 1..20-i {
            print!("*");
         }
         print!("\n");
       }
    }
}


fn ali(original_text: &String) {
    println!("Ali says: {}", original_text);
}
fn hossein(text: &String) {
    println!("{} hossein", text);
}
fn mohammad(original_input: &mut String) {
    original_input.push_str("!");
}