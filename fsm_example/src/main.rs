use fsm_derive::Fsm;

#[derive(Fsm)]
enum MyTest {
    Bla,
    Ble,
}


fn main() {
    let bla = MyTest::Ble;
    println!("Hello, world! {}", bla.name());
    println!("Hello, world! bla={}", bla.bla());
}
