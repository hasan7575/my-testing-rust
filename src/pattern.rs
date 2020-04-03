
fn print_message(user_level: u8) {
    match user_level {
        0 => println!("Welcome dear admin."),
        1 => println!("Welcome back our best member."),
        2 => println!("Hello. Please register first."),
        _ => println!("Bad Input")

    }
}

#[derive(Debug,Clone)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500
}
fn print_status(status:HttpStatus){
    match status {
        HttpStatus::Ok => println!("{:?}  your connect in good",status),
        HttpStatus::NotFound => println!("this page NotFound"),
        HttpStatus::InternalServerError => println!("your connection has InternalServerError"),
        // _ => println!("I cant show your status"),
    }    
}
pub fn run()  {
    let user_inputs: [u8;5] = [0, 1, 2, 3, 4];
    for value in user_inputs.iter() {
        print_message(*value);
    }
    let http_status_list=[HttpStatus::Ok,HttpStatus::NotFound,HttpStatus::InternalServerError];
    
    for value in http_status_list.iter() {
        print_status(value.clone());
    }
}

 
