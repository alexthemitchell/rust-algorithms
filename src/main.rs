mod search;

fn main() {
    let arr = [0,2,4];
    let v = 3;
    let i = search::linear(v,&arr);
    println!("Found {} in {:?} at {:?}", v, arr, i);
    let j = search::binary(v,&arr);
    println!("Found {} in {:?} at {:?}", v, arr, j);
}

