
#[derive(Debug)]
struct point<T,U> {
    x: T,
    y: U,
} 
impl <T,U> point<T,U> {
    // fn x(&self)-> &T {
    //     &self.x
    // } 
    fn mixup<V,W>(self, other: point<V, W>) -> point<T,W> {
        point {
            x : self.x,
            y : other.y,
        }
    }
}
fn main(){
    let int_point = point{x:23,y:54};
    //let float_point = point{x:10.5,y:12.2};
    let char_point = point{x:" hello ",y:" world "};
    // println!("{:?}",int_point.x());
    // println!("{:?}",float_point);

    let result = int_point.mixup(char_point);
    println!("{:?}",result);
    

}