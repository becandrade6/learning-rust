fn main() {

    // this is an intentional panic call
    //panic!("crash and burn");


    let v = vec![1, 2, 3];

    // this causes a panic because we accessing an array beyond the end
    v[99];
}
