use bits128::Bits128;

fn main() {
    let mut a = Bits128::from_dec(50001); // 1100001101010001
    dbg!(a.last_bit());
    dbg!(a.first_bit());
    dbg!(a.at(7)); // false
    dbg!(a.at(8)); // true
    a.flip(11);
    dbg!(a); // now it should be 52049 == 1100101101010001
    println!("{}", a);
    println!("{}", a[5]);
    println!("{}", a[4]);

    assert_eq!(10, Bits128::from_dec(1023).into_iter().fold(0, |sum, x| sum + x as u8  ));
}