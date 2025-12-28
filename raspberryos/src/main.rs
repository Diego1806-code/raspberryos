mod test;
mod test1 {
    pub mod testnest;
}
fn main() {
    println!("Hello, world!");
    test::funct();
    test1::testnest::test();
}
