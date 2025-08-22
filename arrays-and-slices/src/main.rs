// fn prnt_slice(slice: &[i32]) {
//     println!("{:?}", slice)
// }

// fn main() {
//     let mut arr: [i32; 10] = [0; 10];

//     arr[0] = 1;
//     let f: Option<&i32> = arr.get(10);

//     let mut y = &mut arr[2..8];

//     y[1] = 2;
//     prnt_slice(y);
// }

fn print_slice(s: &[i32]) {
    for val in s {
        println!("{}", val);
    }
}

fn main() {
    let mut arr: [i32; 5] = [10, 20, 30, 40, 50];
    let slice = &mut arr[1..4];
    slice[0] = 100;
    print_slice(slice);
}
