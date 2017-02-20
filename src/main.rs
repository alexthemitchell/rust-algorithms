mod search;
mod tests;
extern crate time;

fn main() {
    println!("Testing Linear Search...");
    tests::test(search::linear);
    println!("Testing Binary Search...");
    tests::test(search::binary);
}

//fn benchmark(v, arr) {
//    let before = time::precise_time_ns();
//    let i = search::linear(v,&arr);
//    let after = time::precise_time_ns(); 
//}
