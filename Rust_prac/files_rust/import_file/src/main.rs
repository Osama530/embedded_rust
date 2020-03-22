use std::fs::File;
use std::io;
fn main() {
    let mut file = File::open("osama.txt");
                            //.expect("fail to open files");
    let mut content = String::new();
    file.read_to_string(&mut content);
                            //.expect("fail to load content");
    println!("content of file is: \n\n {}",content);

}
