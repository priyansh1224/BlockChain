// fn main(){
//     // println!("Hello World");

//     let a = 10;
//     let b= 20;
//     let ans = add_number(a,b);
    
//     let str = String::from("Hello");

//     finder(str);
//     println!("{}",ans);
//     println!("{}",a);
// }



// fn add_number(x:i32,y:i32)-> i32{
//    x+y
// }

// fn finder(s: String){
//     println!("{}",s);

//     // drop
// }



fn main(){
//     let mut str = String::from("Strike is Coming");


//     // let r1 = &str;
//     // let r2 = &str;
    

//     // println!("{} {}",r1,r2);
//     let r2 = & str;
//     // 
//     let r1 = &mut str;
    
//     // let r2 = &mut str;
    
//     // data ko read kar sakta hu str
//     //  println!("{} {}",str);

//     println!("{}",r1);
// //    borrowing wapis kar do
//     println!("{}",str);
    
    // // only read the data
    // immutable_borrow(&str);


    // // mutable_borrwoing(Read and write)
    // muttable_borrow(&mut str);

    // println!("{}",str);


    // let mut arr = [10,20,30,40,50];
    // // println!("{}",arr[0]);
    // arr[2] = 70;

    // for element in arr{
    //     println!("{}",element);
    // }


    // for num in (1..=5).rev(){
    //     println!("{}",num);
    // }

    // println!("Hello");

    // let age = 15;

    // if age >= 18 {
    //     println!("You are eligible for vote");
    // }
    // else if age == 15{
    //     println!("You are hero");
    // }
    // else{
    //     println!("You are not eligible for vote");
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;
    //     println!("Again!");

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("{}",result);

    // tuple

    let a = (10,20,true,"Rohit");

    println!("{}",a.2);

}


// fn immutable_borrow(s: &String){
//     println!("{}",s);
// }

// fn muttable_borrow(s: &mut String){

//     s.push_str(" Hi");
//     println!("{}",s);
// }
