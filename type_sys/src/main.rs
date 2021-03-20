fn main() {
    // stack-alloc
    let arr: [i32; 3] = [1, 2, 3];

    // heap-alloc
    let boxed_arr: Box<[i32]> = Box::new([1, 2, 3]);

    let slice: &[i32] = &boxed_arr[..];

    fn foo<T>() {}
    let x = &mut foo::<i32>;
    let foo_ptr_1: fn() = foo::<i32>;

    let want_i32 = true;
    let foo_ptr_2 = if want_i32 { foo::<i32> } else { foo::<u32> };

    fn f<F: FnOnce() -> String>(g: F) {
        println!("{}", g());
    }

    let mut s = String::from("foo");
    let t = String::from(" bar");
    f(|| {
        s += &t;
        s
    });

    // function pointer type:
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    let mut x = add(5, 7);
    println!("add res is {:?}", x);

    type Binop = fn(i32, i32) -> i32;
    let bo: Binop = add;
    x = bo(4, 8);
    println!("add function pointer type res is {:?}", x);

    // trait object
    // Trait
    // dyn Trait
    // dyn Trait + Send
    // dyn Trait + Send + Sync
    // dyn Trait + 'static
    // dyn Trait + Send + 'static
    // dyn 'static + Trait
    // dyn (Trait)

    trait Printable {
        fn stringify(&self) -> String;
    }
    impl Printable for i32 {
        fn stringify(&self) -> String {
            self.to_string()
        }
    }

    fn print(a: Box<dyn Printable>) {
        println!("{}", a.stringify());
    }

    print(Box::new(10) as Box<dyn Printable>);

    // Type parameters
    /*
     *
     *     fn to_vec<A: Clone>(xs: &[A]) -> Vec<A> {
     *         if xs.is_empty() {
     *             return vec![];
     *         }
     *         let first: A = xs[0].clone();
     *         let mut rest: Vec<A> = to_vec(&xs[1..]);
     *         rest.insert(0, first);
     *         rest
     *     }
     */

    // inferred type
    let x: Vec<_> = (0..10).collect();

    // type layout
    #[repr(C)]
    struct ThreeInts {
        first: i16,
        second: i8,
        third: i32,
    }
    println!("ThreeInts size is {:?}", std::mem::size_of::<ThreeInts>());
    println!("ThreeInts align is {:?}", std::mem::align_of::<ThreeInts>());

    #[repr(packed(2))]
    struct PackedStruct {
        first: i16,
        second: i8,
        third: i32,
    }
    println!(
        "PackedStruct size is {:?}",
        std::mem::size_of::<PackedStruct>()
    );
    println!(
        "PackedStruct align is {:?}",
        std::mem::align_of::<PackedStruct>()
    );

    #[repr(C, align(8))]
    struct AlignedStruct {
        first: i16,
        second: i8,
        third: i32,
    }
    println!(
        "AlignedStruct size is {:?}",
        std::mem::size_of::<AlignedStruct>()
    );
    println!(
        "AlignedStruct align is {:?}",
        std::mem::align_of::<AlignedStruct>()
    );

    union Union {
        f1: u16,
        f2: [u8; 4],
    }
    println!("Union size is {:?}", std::mem::size_of::<Union>());
    println!("Union align is {:?}", std::mem::align_of::<Union>());

    union SizeRoundUp {
        a: u32,
        b: [u16; 3],
    }
    println!(
        "SizeRoundUp size is {:?}",
        std::mem::size_of::<SizeRoundUp>()
    );
    println!(
        "SizeRoundUp align is {:?}",
        std::mem::align_of::<SizeRoundUp>()
    );

    #[repr(C)]
    enum MyEnum {
        A(u32),
        B(f32, u64),
        C { x: u32, y: u8 },
        D,
    }

    println!("MyEnum size is {:?}", std::mem::size_of::<MyEnum>());
    println!("MyEnum align is {:?}", std::mem::align_of::<MyEnum>());

    #[repr(C)]
    enum MyEnumDiscriminant {
        A,
        B,
        C,
        D,
    }
    println!(
        "MyEnumDiscriminant size is {:?}",
        std::mem::size_of::<MyEnumDiscriminant>()
    );
    println!(
        "MyEnumDiscriminant align is {:?}",
        std::mem::align_of::<MyEnumDiscriminant>()
    );

    #[repr(C)]
    union MyEnumFileds {
        A: MyAFileds,
        B: MyBFileds,
        C: MyCFileds,
        D: MyDFileds,
    }
    println!(
        "MyEnumFileds size is {:?}",
        std::mem::size_of::<MyEnumFileds>()
    );
    println!(
        "MyEnumFileds align is {:?}",
        std::mem::align_of::<MyEnumFileds>()
    );

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct MyAFileds(u32);

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct MyBFileds(f32, u64);

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct MyCFileds {
        x: u32,
        y: u8,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct MyDFileds;

    println!("MyAFileds size is {:?}", std::mem::size_of::<MyAFileds>());
    println!("MyAFileds align is {:?}", std::mem::align_of::<MyAFileds>());

    println!("MyBFileds size is {:?}", std::mem::size_of::<MyBFileds>());
    println!("MyBFileds align is {:?}", std::mem::align_of::<MyBFileds>());

    println!("MyCFileds size is {:?}", std::mem::size_of::<MyCFileds>());
    println!("MyCFileds align is {:?}", std::mem::align_of::<MyCFileds>());

    println!("MyDFileds size is {:?}", std::mem::size_of::<MyDFileds>());
    println!("MyDFileds align is {:?}", std::mem::align_of::<MyDFileds>());

    #[repr(C)]
    union MyEnumFileds2 {
        A: MyAFileds2,
        B: MyBFileds2,
        C: MyCFileds2,
        D: MyDFileds2,
    }
    println!(
        "MyEnumFileds2 size is {:?}",
        std::mem::size_of::<MyEnumFileds2>()
    );
    println!(
        "MyEnumFileds2 align is {:?}",
        std::mem::align_of::<MyEnumFileds2>()
    );

    #[repr(u8)]
    #[derive(Copy, Clone)]
    enum MyEnumDiscriminant2 {
        A,
        B,
        C,
        D,
    }

    println!(
        "MyEnumDiscriminant2 size is {:?}",
        std::mem::size_of::<MyEnumDiscriminant2>()
    );
    println!(
        "MyEnumDiscriminant2 align is {:?}",
        std::mem::align_of::<MyEnumDiscriminant2>()
    );

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct MyAFileds2(MyEnumDiscriminant2, u32);

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct MyBFileds2(MyEnumDiscriminant2, f32, u64);

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct MyCFileds2 {
        x: u32,
        y: u8,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct MyDFileds2;

    println!("MyAFileds2 size is {:?}", std::mem::size_of::<MyAFileds2>());
    println!(
        "MyAFileds2 align is {:?}",
        std::mem::align_of::<MyAFileds2>()
    );

    println!("MyBFileds2 size is {:?}", std::mem::size_of::<MyBFileds2>());
    println!(
        "MyBFileds2 align is {:?}",
        std::mem::align_of::<MyBFileds2>()
    );

    println!("MyCFileds2 size is {:?}", std::mem::size_of::<MyCFileds2>());
    println!(
        "MyCFileds2 align is {:?}",
        std::mem::align_of::<MyCFileds2>()
    );

    println!("MyDFileds2 size is {:?}", std::mem::size_of::<MyDFileds2>());
    println!(
        "MyDFileds2 align is {:?}",
        std::mem::align_of::<MyDFileds2>()
    );

    struct PrintOnDrop(&'static str);
    impl Drop for PrintOnDrop {
        fn drop(&mut self) {
            println!("{}", self.0);
        }
    }

    let mut overwritten = PrintOnDrop("Drops when overwritten");
    overwritten = PrintOnDrop("Drops when scope ends");

    let tuple = (PrintOnDrop("Tuple first"), PrintOnDrop("Tuple second"));
    let moved;
    moved = PrintOnDrop("Drops when moved");
    moved;

    let uninitialized: PrintOnDrop;
    let mut partial_move = (PrintOnDrop("first"), PrintOnDrop("second"));
    core::mem::forget(partial_move.1);

    fn patterns_in_param((x, _): (PrintOnDrop, PrintOnDrop), (_, y): (PrintOnDrop, PrintOnDrop)) {}
    patterns_in_param(
        (PrintOnDrop("0"), PrintOnDrop("1")),
        (PrintOnDrop("2"), PrintOnDrop("3")),
    );

    let local_var = PrintOnDrop("local var");
    if PrintOnDrop("If condition").0 == "If condition" {
        PrintOnDrop("If body").0
    } else {
        unreachable!();
    };

    PrintOnDrop("frist operand").0 == ""
        || PrintOnDrop("second operand").0 == ""
        || PrintOnDrop("third operand").0 == "";

    match PrintOnDrop("Matched value in final exression") {
        _ if PrintOnDrop("guard condition").0 == "" => (),
        _ => (),
    }

    loop {
        (
            PrintOnDrop("Outer tuple first"),
            PrintOnDrop("Outer tuple second"),
            (
                PrintOnDrop("Inner tuple first"),
                PrintOnDrop("Inner tuple second"),
                break,
            ),
            PrintOnDrop("Never created"),
        );
    }
    /*
        let x = &temp();
        let x = &temp() as &dyn Send;
        let x = (&*&temp(),);
        let x = { [Some { 0: &temp() }] };
        let ref x = temp();
        let ref x = *&temp();

        // life elision
        fn print1(s: &str);
        fn print2(s: &'_ str);
        fn print3<'a>(s: &'a str);

        fn debug1(lvl: usize, s: &str);
        fn debug2<'a>(lvl: usize, s: &'a str);

        fn substr1(s: &str, until: usize) -> &str;
        fn substr2<'a>(s: &'a str, until: usize) -> &'a str;

        fn get_mut1(&mut self) -> &mut dyn T;
        fn get_mut2<'a>(&'a mut self) -> &'a mut dyn T;

        fn args1<T: ToCStr>(&mut self, args: &[T]) -> &mut Command;
        fn args2<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command;

        fn new1(buf: &mut [u8]) -> Thing<'_>;
        fn new2(buf: &mut [u8]) -> Thing;
        fn new3<'a>(buf: &'a mut [u8]) -> Thing<'a>;
    */

    type FunPtr1 = fn(&str) -> &str;
    type FunPtr2 = for<'a> fn(&'a str) -> &'a str;

    type FunTrait1 = dyn Fn(&str) -> &str;
    type FunTrait2 = dyn for<'a> Fn(&'a str) -> &'a str;

    trait Foo {}
    type T1 = Box<dyn Foo>;
    type T2 = Box<dyn Foo + 'static>;

    impl dyn Foo {}
    impl dyn Foo + 'static {}

    type T3<'a> = &'a dyn Foo;
    type T4<'a> = &'a (dyn Foo + 'a);

    struct TwoBounds<'a, 'b, T: ?Sized + 'a + 'b> {
        f1: &'a i32,
        f2: &'b i32,
        f3: T,
    }

    trait Bar<'a>: 'a {}
    type T11<'a> = Box<dyn Bar<'a>>;
    type T12<'a> = Box<dyn Bar<'a> + 'a>;

    impl<'a> dyn Bar<'a> {}
    impl<'a> dyn Bar<'a> + 'a {}

    const STRING: &str = "bitstring";

    struct BitsNStrings<'a> {
        mybits: [u32; 2],
        mystrings: &'a str,
    }

    const BITS_N_STRINGS: BitsNStrings<'_> = BitsNStrings {
        mybits: [1, 2],
        mystrings: STRING,
    };

    const RESOLVED_SINGLE: fn(&str) -> &str = |x| x;

    // Special Types and Traits
    // Box<T>
    // Rc<T>
    // Arc<T>
    // Pin<P>
    // UnsafeCell<T>
    // PhantomData<T>
    // Operator Traits
    // Deref and DerefMut
    // Drop
    // Copy
    // Clone
    // Send
    // Sync
    // Auto Traits: Send,Sync, Unpin, UnwindSafe, RefUnwindSafe,

    fn random_bool() -> bool {
        true
    }
    fn initialization_example() {
        let init_after_if: ();
        let uninit_after_if: ();

        if random_bool() {
            init_after_if = ();
            uninit_after_if = ();
        } else {
            init_after_if = ();
        }
        init_after_if;
        // uninit_after_if; // use of possibly uninitializated
    }

    // link
    //
    #[cfg(target_feature = "crt-static")]
    fn f1() {
        println!("the C runtime should be statically linked");
    }

    #[cfg(not(target_feature = "crt-static"))]
    fn f2() {
        println!("the C runtime should be dynamically linked");
    }

    use std::env;
    let linkage = env::var("CARGO_CFG_TARGET_FEATURE").unwrap();
    if linkage.contains("crt-static") {
        println!("the C runtime should be statically linked");
    } else {
        println!("the C runtime should be dynamically linked");
    }
}
