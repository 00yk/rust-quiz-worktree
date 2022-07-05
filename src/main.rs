mod quiz1;
mod quiz2;
fn main() {
    // let a = f();
    // println!("{:?}", a);
    let x = || { (return) || true; };
    println!("{:?}", x());
}
