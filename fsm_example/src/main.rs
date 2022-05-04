use fsm_derive::Fsm;

#[derive(Fsm)]
enum MyTest {
    Bla,
    Ble,
}


fn main() {
    let ble = MyTest::Ble;
    let bla = MyTest::Bla;
    println!("Hello, world! {}", bla.name());
    println!("Hello, world! bla={}", bla.bla());
    println!("Hello, world! ble={}", ble.bla());
}
