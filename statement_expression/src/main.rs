//
//
fn main() {
    // declaration statements
    // expression statements
    // item declarations
    fn outer() {
        let outer_var = true;
        // fn inner() {
        // println!("outer var is {:?}", outer_var);
        // outer_var is not in scope here
        // }
        // inner();
    }

    // ignore else will cause type mismacth i32 and ()
    if true {
        1i32
    } else {
        2i32
    };

    //let statements

    fn f1() {}
    let _: () = {
        f1();
    };

    let five = {
        f1();
        5
    };
    println!("five is {}", five);

    struct S1;
    impl S1 {
        fn consume_self(self) {
            println!("consume self");
        }
        fn borrow_self(&self) {
            println!("borrow self");
        }
    }

    fn move_by_block_expression() {
        let s = S1;
        (s).borrow_self();
        // (&{ s }).borrow_self(); // block exp move out of s
        s.consume_self();
    }

    move_by_block_expression();

    unsafe {
        let b = [13u8, 17u8];
        let a = &b[0] as *const u8;
        println!("a is {:?}", *a);
        println!("a offset 1is {:?}", *a.offset(1));
    }

    fn is_unix_platform() -> bool {
        #[cfg(unix)]
        {
            true
        }
        #[cfg(not(unix))]
        {
            false
        }
    }
    println!("is unix platform: {:?}", is_unix_platform());

    {
        let shared_ref = &8;
    }

    let mut arr = [1, 2, 3];

    {
        let mut_ref = &mut arr;
    }

    let aa = &&10;
    let bb = &&10;
    let cc = &&mut 10;
    println!("a,b,c is {},{},{}", *aa, *bb, *cc);

    let xx = &7;
    println!("xx is {:?}", *xx);

    let yy = &mut 9;
    *yy = 10;
    println!("yy is {:?}", *yy);

    use std::num::ParseIntError;
    fn try_to_parse() -> Result<i32, ParseIntError> {
        let x: i32 = "123".parse()?;
        let y: i32 = "24a".parse()?;
        Ok(x + y)
    }

    let res = try_to_parse();
    println!("res is {:?}", res);

    fn try_option_name() -> Option<u8> {
        let val = Some(1)?;
        Some(val)
    }
    println!("op some is {:?}", try_option_name());

    fn try_option_none() -> Option<u8> {
        let val = None?;
        Some(val)
    }
    println!("op some is {:?}", try_option_none());

    #[warn(unconditional_panic)]

    println!("[1, 2, 3, 4] second is {:?}", ([1, 2, 3, 4])[2]);
    println!("b12 = {:?}", [[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    // println!("x is {:?}", ["a", "b"][10]); // compiler out of range error

    let pair = ("a string", 2);
    println!("0 = {:?}, 1 = {:?}", pair.0, pair.1);

    // struct Point { x: f64, y: f64, };
    struct Point(f64, f64);

    let point = Point(1.0, 0.0);
    println!("p0 is {:?}, p1 is {:?}", point.0, point.1);

    // struct expression
    struct S2 {
        x: f64,
        y: f64,
    };

    struct S3 {};
    struct S4(f64, f64);
    struct S5 {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut base = S5 { x: 1, y: 2, z: 3 };
    println!("S5 x={},y={},z={}", base.x, base.y, base.z);
    // println!("S5 x={},y={},z={}", S5::x, S5::y, S5::z);
    let y_ref = &mut base.y;
    S5 {
        y: 0,
        z: 10,
        ..base
    };
    drop(y_ref);
    println!("S5 x={},z={}", base.x, base.z);

    #[derive(Debug)]
    struct Color(u8, u8, u8);
    let c1 = Color(126, 127, 128);
    let c2 = Color {
        0: 255,
        1: 255,
        2: 0,
    };
    let c3 = Color { 1: 0, ..c1 };
    println!("c1 is {:?}", c1);
    println!("c2 is {:?}", c2);
    println!("c3 is {:?}", c3);

    // tuple struct expression

    #[derive(Debug)]
    struct Pos(i32, i32, i32);

    Pos(1, 2, 3);
    let pos2 = Pos; // pos2 is a function takes 3 arguments
    let pos3 = pos2(8, 9, 10); // creates a Pos value

    println!("pos2 = {:?}", pos2(5, 6, 7));
    println!("pos3 = {:?}", pos3);

    // unit struct expression
    struct Gramma;
    let S8 = Gramma;
    let S9 = Gramma {};

    // call expression
    trait Pretty {
        fn print(&self);
    }

    trait Ugly {
        fn print(&self);
    }

    struct S10;
    impl Pretty for S10 {
        fn print(&self) {
            println!("S10 Pretty");
        }
    }

    struct S11;
    impl Pretty for S11 {
        fn print(&self) {
            println!("S11 Pretty");
        }
    }

    impl Ugly for S11 {
        fn print(&self) {
            println!("S11 Ugly");
        }
    }

    let s10 = S10;
    let s11 = S11;

    s10.print();
    S10::print(&s10);
    <S10 as Pretty>::print(&s10);

    // s11.print(); ambiguating function
    // S11.print(&s11);
    <S11 as Pretty>::print(&s11);
    <S11 as Ugly>::print(&s11);

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let three: i32 = add(1i32, 2i32);
    println!("three is {:?}", three);

    let name: &'static str = (|| "Rust")();
    println!("call fn is {:?}", name);

    // closure expression
    fn ten_times<F>(f: F)
    where
        F: Fn(i32),
    {
        for index in 0..10 {
            f(index);
        }
    }

    ten_times(|j| println!("hello, {}", j));
    ten_times(|j: i32| -> () { println!("hello new, {}", j) });
    let word = "konnichiwa".to_owned();
    ten_times(move |j| println!("{}, {}", word, j));

    // loop expression
    let ret = loop {
        println!("I live.");
        break 8u32;
    };

    let mut i = 0;
    while i < 10 {
        println!("hello loop");
        i = i + 1;
    }

    let mut x = vec![1, 2, 3, 4];
    while let Some(y) = x.pop() {
        println!("y = {}", y);
    }

    while let _ = 5 {
        println!("Irrefutable patterns are always true");
        break;
    }

    // 'label: while let PATS = EXPR {
    /*
     * loop body
     */
    // }
    //
    // 'label: loop {
    //     match EXPS {
    //         PATS => { [>loop body<] }
    //         _ => break,
    //     }
    // }
    /*  */

    let mut vals = vec![2, 3, 1, 2, 2, 3];
    while let Some(v @ 1) | Some(v @ 2) = vals.pop() {
        println!("{}", v);
    }

    let v = &["apples", "cake", "coffee"];
    for text in v {
        println!("I like {}", text);
    }

    let mut sum = 0;
    for n in 1..11 {
        sum += n;
    }
    println!("sum is {:?}", sum);

    let mut last = 0;
    for x in 1..100 {
        if x > 12 {
            break;
        }
        last = x;
    }
    println!("last is {:?}", last);

    let (mut a, mut b) = (1, 1);
    let break_res = loop {
        if b > 10 {
            break b;
        }
        let c = a + b;
        a = b;
        b = c;
    };

    println!("break result is {:?}", break_res);

    // if expression
    let x: u32 = 3;
    if x == 4 {
        println!("x is four");
    } else if x == 3 {
        println!("x is three");
    } else {
        println!("x is something else");
    }

    let y = if 13 * 15 > 150 { "Bigger" } else { "Smaller" };

    let dish = ("Ham", "Eggs");
    if let ("Bacon", b) = dish {
        println!("Bacon is served with {:?}", b);
    } else {
        println!("No bacon will be served");
    }

    if let ("Ham", b) = dish {
        println!("Ham is served with {:?}", b);
    } else {
        println!("No Ham will be served");
    }

    let x = Some(3);
    let a = if let Some(1) = x {
        1
    } else if x == Some(2) {
        2
    } else if let Some(y) = x {
        y
    } else {
        -1
    };
    println!("a is {:?}", a);

    // match expression
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("others"),
    }

    let x = 9;
    let msg = match x {
        0 | 1 => "not many",
        2..=9 => "a few",
        _ => "lots",
    };
    println!("msg is {:?}", msg);

    struct S(i32, i32);

    match S(1, 2) {
        S(z @ 1, _) | S(_, z @ 2) => println!("Matched Struct, z = {:?}", z),
        _ => panic!(),
    }

    let xd = Some(8);
    let msg = match xd {
        Some(x) if x < 10 => println!("smaller than 10"),
        Some(x) => println!("others"),
        None => println!("none"),
    };
    println!("msg is {:?}", msg);
}
