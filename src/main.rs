fn main() {
    let nilai: u8 = 255;
    let nilai2: i8 = -128;
    let tup : (u8, u8, f32) = (200, 0b00000101, 7.8); // data tuple (masing2 data bisa berbeda tipe datanya/beda dgn array)
    let (_a, _b, _c) = tup; // cara I tuple
    let _dua_ratus = tup.0; // cara II tuple
    let _data_biner = tup.1;
    let _pecahan = tup.2;
    let _ab: [u8; 5] = [1, 2, 3, 4, 5];  // data array
    let _ab_array = _ab[3]; //cara akses array

    //menambahkan komen
    println!("nilai 1 : {nilai}");
    println!("nilai 2 : {nilai2}");
    println!("nilai 3 : {_a}, {_b}, {_c}");
    println!("nilai 4 : {_dua_ratus}");
    println!("nilai 4 : {_data_biner}");
    println!("nilai 4 : {_pecahan}");
    println!("nilai aray : {_ab_array}");
    
}

// *************************** contoh 
// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }