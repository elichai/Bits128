use bits128::Bits128;

fn main() {
    let mut a = Bits128::from_dec(50001); // 1100001101010001
    dbg!(a.last());
    dbg!(a.first());
    dbg!(a.at(7)); // false
    dbg!(a.at(8)); // true
    a.flip(11);
    dbg!(a); // now it should be 52049 == 1100101101010001
//    println!("{:?}", &a.arr()[..]);
    println!("{}", a);
    println!("{}", a[5]);
    println!("{}", a[4]);
}