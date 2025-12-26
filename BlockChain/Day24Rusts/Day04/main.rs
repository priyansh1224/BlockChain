fn main(){
    // let a:&str = "Rohit";
    
    // println!("{}",a);
    // println!("{}",a);

    // let mut a = [10,30,50];
    // a[0] = 70;
    
    // println!("{:?}",a);


    // vector

    // let mut arr: Vec<i32> = Vec::new();

    // let mut arr = vec![10,30,70];
    // arr.push(30);
    // arr.push(60);
    // arr.push(10);
    // println!("{:?}",arr);

    // println!("{}",arr.len());
    // println!("{}",arr.capacity());
    // println!("{:p}",arr.as_ptr());
    // println!("{:p}",&arr);
    // println!("{:p}",&arr[0]);
    // println!("{:p}",&arr[1]);


    // println!("{}",arr[1]);

    // for num in 0..=5{
    //     println!("{}",arr[num]);
    // }
    
    // for num in arr{
    //     println!("{}",num);
    // }

    // for num in &mut arr{
    //     *num +=2;
    //     println!("{}",num);
    // }

    // println!("{}",arr[0]);


    // let mut v: Vec<String> = Vec::new();

    // v.push(String::from("Rohit"));
    // v.push(String::from("Negi"));
    // v.push(String::from("Strike"));
    // v.push(String::from("Gopal"));
    
    // v.insert(2,String::from("Manish Ji"));
    // println!("{:?}",v);
    //  println!("{:?}",v.get(1));


    let mut v = vec![30,70,90,100];

    // let a: &[i32] = &v[1..3];
    let sum = add_number(&mut v[0..4]);

     println!("{}",sum);
}

// let array_slice: &[i32] = &array;
// let array_slice: &Vec<i32> = &array;


// fn add_number(arr: &[i32])->i32{
   
//    let mut sum = 0;
//    for num in arr{
//     sum+=num;
//    }

//    sum
// }