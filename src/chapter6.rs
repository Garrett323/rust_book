/*
 * HAVING FUN WITH ENUMS
 * and pattern matching too
 */

pub fn run() {
    create();
}

enum MyEnum {
    A,
    B,
}

// enums also can contain information
enum MyEnumButBetter {
    A(u32),
    B(i32),
}

fn create() {
    let a = MyEnum::A;
    let b = MyEnum::B;
    test(a);
    test(b);
}

fn with_data() {
    let a = MyEnumButBetter::A(30);
    let b = MyEnumButBetter::B(-30);
}

fn test(_: MyEnum) {}
