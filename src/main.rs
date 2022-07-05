mod quiz1;
mod quiz2;
mod quiz3;
fn main() {
    // let a = f();
    // println!("{:?}", a);
    let x = || { (return) || true; };
    println!("{:?}", x());
}
