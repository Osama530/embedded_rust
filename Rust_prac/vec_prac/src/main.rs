use std::io;
fn main() {
    //let mut vector: Vec<String> = Vec::new();

    // for i in (0..3){
    //     println!("Enter an integer character {}: ",i);
    //     let mut num = String::new();
    //     io::stdin().read_line(&mut num);
    //     let num:String = num.trim().parse().unwrap();

    //     vector.push(num);
    // }
    // println!("vactor = {:?}",vector[3]);

    // let collection = vec!["Khizar", "osama", "asgher", "Farrukh"];
    // for i in 0..3 {
    //     println!("{}",collection[i]);
    //     for j in collection[1].chars() {
    //          println!("{}",j);
    //     }
    // }

    println!("Enter an String: ");
    let mut vector: Vec<String> = Vec::new();
    let mut data = String::new();
    io::stdin().read_line(&mut data);
    let data:String = data.trim().parse().unwrap();

    vector.push(data);
    let mut count = 0;
    let mut vov_count = 0;
    let mut con_count = 0;
    let mut space_count = 0;
    println!("{:?}",vector);
    for i in vector[0].chars() {
        count+=1;
        match i {
            ' ' => space_count+=1,
            'a' => vov_count+=1,
            'e' => vov_count+=1,
            'i' => vov_count+=1,
            'o' => vov_count+=1,
            'u' => vov_count+=1,
            _   => con_count+=1,
        }
    }
    println!("{}",count);
    println!("{}",vov_count);
    println!("{}",con_count);
    println!("{}",space_count);




     


}
