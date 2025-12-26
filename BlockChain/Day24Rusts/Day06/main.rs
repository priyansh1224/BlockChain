// // // enum state{
// // //     connecting,
// // //     disconnected,
// // //     connected
// // // }


// // fn main(){
// //     let systemConnection = state::disconnected;


// //     match systemConnection{
// //         state::connected => println!("Your system is connected"),
// //         state::disconnected => println!("Your system is dissconnected"),
// //         state::connecting => println!("I am connecting")
// //     };
// // }



// // enum Traffic{
// //     Red,
// //     Green,
// //     Yellow
// // }


// // // fn main(){
// // //     let light = Traffic::Green;

// // //     let update = match light{
// // //         Traffic::Red => String::from("You need to wait"),
// // //         Traffic::Green => String::from("You can go"),
// // //         Traffic::Yellow => String::from("Please stop"),
// // //     };

// // //     println!("{}",update);
// // // }


// // enum Message{
// //     Quit,   // No data
// //     Text(String,i32,i32),  //Tuples
// //     Move{x:i32,y:i32},  //Structure
// // }


// // fn matching(msg: &Message){
// //     match msg{
// //         Message::Quit => println!("There is no message"),
// //         // _ => {},
// //     }
// // }

// // fn main(){
// //     let msg1 = Message::Text(String::from("Rohit Negi"),30,70);
// //     let msg2 = Message::Quit;
// //     let msg3 = Message::Move{x:20, y:40};

// //     matching(&msg1);
// //     matching(&msg2);
// //     matching(&msg3);
// // }

// // let x:i32 = 10;


// // enum Option<T,E>{
// //     Some(T),
// //     None,
// //     Hell{x:E,y:E},
// // }

// // fn main(){
// //     let msg2:Option<String,i32> = Option::Some(String::from("Hello Ji"));
// //     let msg1:Option<String,i32> = Option::Hell{x:40,y:90};
// //     let msg3:Option<String,i32> = Option::None;

// //     match msg1{
// //         Option::Some(value) => println!("{}",value),
// //         Option::None => println!("Nothing to show"),
// //         Option::Hell{x,y} => println!("{} {}",x,y),
// //     }

// //     match msg2{
// //         Option::Some(value) => println!("{}",value),
// //         Option::None => println!("Nothing to show"),
// //         Option::Hell{x,y} => println!("{} {}",x,y),
// //     }
// // }



// enum Result<T,E>{
//     Ok(T),
//     Err(E)
// }


// fn divide(x:i32,y:i32) -> Result<i32,String> {
//     if y==0 {
//         return Result::Err(String::from("Division is not possible"));
//     }
//     else{
//         return Result::Ok(x/y);
//     }
// }


// fn main(){
//     let result = divide(10,11);

//     match result{
//         Result::Ok(value) => println!("{}",value),
//         Result::Err(error) => println!("{}",error),
//     }
// }


enum Traffic{
    Red,
    Yellow,
    Green
}

impl Traffic{
   
//    constructor: value initilise

  fn new() ->Self{
    Traffic::Red
  }
  

//   
   fn next(&self)-> Traffic{
     match self{
        Traffic::Red => Traffic::Green,
        Traffic::Green => Traffic::Yellow,
        Traffic::Yellow => Traffic::Red
     }
   }

//. duration

    fn duration(&self) ->u32{
        match self{
        Traffic::Red => 60,
        Traffic::Green => 45,
        Traffic::Yellow => 10,
        }
    }  


    // can cross

    fn can_cross(&self)-> bool{
        match self{
            Traffic::Green => true,
            _ => false
        }
    }  



}


fn main(){
    
    let mut light = Traffic::new();

    // light = light.next();
 
    let dur = light.duration();
    
    println!("{}",dur);
    println!("{}",light.can_cross());
    

}




