use fsm_derive::Fsm;

#[derive(Fsm)]
enum MyTest {
    Bla,
    Ble,
    X(u32, u32, i8),
}

#[derive(Fsm)]
enum Test2 {
    A(u32, u32, i8),
    B,
    C {name: String, age: u32},
}


fn main() {
    let ble = MyTest::Ble;
    let bla = MyTest::Bla;
    let x = MyTest::X(5_u32, 7_u32, 7_i8);
    println!("Hello, world! {}", bla.name());
    println!("Hello, world! bla={}", bla.bla());
    println!("Hello, world! ble={}", ble.bla());
    println!("Hello, world! x={}", x.bla());
}
