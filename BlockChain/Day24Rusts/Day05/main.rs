// struct Book{
//     name:String,
//     author:String,
//     page_count:i32,
//     price:i32
// }

// // Formula baseaddress+ index*sixeofData


// fn main(){
//     let book1 = Book{
//         name:String::from("How to friends and influence people"),
//         author:String::from("Dale Carneggie"),
//         page_count:200,
//         price:500
//     };

//     let book2 = Book{
//         name:String::from("Rich Dad and Por Dad"),
//         author:String::from("Robert"),
//         page_count:100,
//         price:200
//     };

//     let book3 = Book{
//         name:String::from("Do belo ki katha"),
//         author:String::from("Munsi prem chandra"),
//         page_count:200,
//         price:1000
//     };


//     let books = vec![book1,book2,book3];

//     for temp in &books {
//         println!("name: {}, author:{}, pageCount:{}, price:{}",temp.name,temp.author,temp.page_count,temp.price);
//     }



//     // tuple 

//     // let a = (String::from("How to win friends and influence people"),String::from("Dale Carneggie"),200,500);

//     // println!("{}",a.2);
    

  
//     // println!("{}",book1.name);
//     // println!("{}",book1.author);
//     // println!("{}",book1.page_count);
//     // println!("{}",book1.price);



    



// }


struct Rectangle{
    width: i32,
    height: i32
}


impl Rectangle{
    
    fn new(width: i32, height: i32)->Self{
        Self {width,height}
    }

    fn area(&self)->i32{
      self.height*self.width
    }

    fn double(&mut self){
        self.height*=2;
        self.width*=2;
    }

    fn compare(&self, other: &Rectangle)->bool{
      
      self.height*self.width>other.height*other.width
    }
}

// self == this





fn main(){
//    let mut r1 = Rectangle{
//     width:100,
//     height:20
//    };



let mut r1 = Rectangle::new(10,20);

   let mut r2 = Rectangle::new(15,18);
   
//    r1.double();
   println!("{}",r1.area());
   println!("{}",r1.compare(&r2));
}


// enum
// Structure Box property ko apply