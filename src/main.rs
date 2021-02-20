//Hello World
// fn main(){
//    println!("Hello World")
// }

//println
// #[derive(Debug)]
// struct Structure(i32);

// #[derive(Debug)]
// struct Deep(Structure);

// fn main() {
//    println!("{} is sha diao", "mao guo rui");

//    println!("{0} {1} {2} {1} ", "mao", "is", "sha");

//    println!("{name} is {some}", name = "mao guo rui ", some = "sha diao");

//    println!(
//       "{} of {:b} people know binary, the other half doesn't",
//       1, 10
//    );

//    println!("{number:>width$}", number = "abc", width = 100);

//    println!("Now {:?} will print!", Deep(Structure(7)));
// }

//Primitives
fn main() {
   let _logical: bool = true;

   let _float: f64 = 1.0;

   let inferred_type;

   inferred_type = 123;

   println!("{:?}", inferred_type);

   let a = 1;
   let b = false;

   let res = reverse((a, b));

   println!("{:?}", res);

   let xs: [i32; 5] = [123321, 2, 3, 4, 5];
   analyze_slice(&xs);

   let xss: [i32;10] = [5; 10];
   analyze_slice(&xss);

   analyze_slice(&xs[2..4]);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
   let (interger, boolean) = pair;

   (boolean, interger)
}

fn analyze_slice(slice: &[i32]) {
   println!("first element of the slice: {}", slice[0]);
   println!("second element of the slice: {}", slice[1]);
   println!("the slice has {} elements", slice.len());
}
