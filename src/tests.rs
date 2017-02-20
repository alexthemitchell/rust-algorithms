// https://github.com/Grinnell-CSC282/binary-search-2017S/blob/master/test.c

pub fn test(search: fn(i32, &[i32])->Result<usize,usize>) {
    let mut tests = 0;
    let mut errors = 0;
    let mut search_set: [i32; 32] = [0;32];

    for i in 0..32 {
        search_set[i] = (i as i32) * 2;
    }

    for size in 0..32 {
        let slice = &search_set[0..size];
        for i in 0..size as i32 {
            tests += 2;
            match search(i*2, slice) {
                Ok(v) => if v != (i as usize) {
                       println!("Found {} at pos {} (expected {}) in array of size {}",i*2,v,i,size);
                       errors += 1;
                    },
                _     => {
                    println!("Failed to find {} at pos {} in array of size {}",i*2,i,size);
                    errors += 1;
                }
            }
            match search(i*2 + 1, slice) {
                Err(e) => if e != (i as usize + 1) {
                    println!("Offered pos {} for {} (expected pos {}) in array of size {}",e,i*2+1, i + 1,size)
                },
                _        => {
                    println!("Incorrect result for {} in array of size {}", i*2 + 1, size);
                    errors += 1;
                }
            }
        }
        tests += 1;
        match search(-1, slice) {
            Err(e) => if e != 0 {
                  println!("Offered pos {} for -1 (expected pos 0) in array of size {}",e,size);
                  errors += 1;
            },
            _       => {
                println!("Incorrect results for -1 in array of size {}.",size)
            }
        }
    }
    println!("{} tests, {} errors.", tests, errors);
}
