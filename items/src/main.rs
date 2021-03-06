//  use declarations
use std::collections::hash_map::{self, HashMap};
use std::option::Option::{None, Some};

/*
 * use self::std::fs as self_fs;
 */
//
// use ::std::fs;
//
/*
 *
 * mod std {
 *     pub mod fs {
 *         pub fn file() {
 *             println!("std fs file open");
 *         }
 *     }
 * }
 *
 */
// module
mod math {
    type Complex = (f64, f64);
    fn sin(f: f64) -> f64 {
        8.0
    }
    fn cos(f: f64) -> f64 {
        8.0
    }
}

fn foo<T>(_: T) {}
fn bar(m1: HashMap<String, usize>, m2: hash_map::HashMap<String, usize>) {
    println!("dec bar");
}

// visibility
mod quux {
    pub use self::foo::{bar, baz};
    pub mod foo {
        pub fn bar() {
            println!("vis bar");
        }
        pub fn baz() {
            println!("vis baz");
        }
    }
}

use crate::foo::baz::foobaz;
use std::path::{self, Path, PathBuf};
mod foo {
    pub mod example {
        pub mod iter {}
    }

    use crate::foo::example::iter;
    // use example::iter;
    use self::baz::foobaz;
    use crate::foo::bar::foobar;

    pub mod bar {
        pub fn foobar() {
            println!("foo::bar::foobar");
        }
    }

    pub mod baz {
        use super::bar::foobar;
        pub fn foobaz() {
            println!("foo::baz::foobaz");
        }
    }
}

// underscore imports

mod fooZoo {
    pub trait Zoo {
        fn zoo(&self) {
            println!("trait Zoo.zoo");
        }
    } // Zoo trait

    impl<T> Zoo for T {}
}

use self::fooZoo::Zoo as _;
struct Zoo; // Zoo struct, underscore import avoids name conflict

macro_rules! ditem{
    ($item: item) => {$item $item}
}

macro_rules! dstmt{
    ($stmt: stmt) => {$stmt $stmt}
}

// function

fn answer_to_life() -> i32 {
    return 42;
}

trait myTrait {
    fn my_func() {}
}

fn foo_gen_func<A, B>(a: A, b: B)
where
    A: myTrait,
    B: myTrait,
{
}

use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

use std::fmt::Debug;
fn foo_gen_func2<T>(x: &[T])
where
    T: Debug,
{
    let fb: f64 = 8.88;
    println!("t type is {:?}", type_of(x));
    println!("&t type is {:?}", type_of(&x));
    println!("t value is {:?}", x);
    println!("fb type is {:?}", type_of(fb));
    println!("&fb type is {:?}", type_of(&fb));
}

// extern "ABI" fn f1() {}
//
/*
 * extern "ABI" {fn f2()}
 * unsafe { f2()}
 *
 */

fn f3() {} // equivalent to extern "Rust" fn f3() {}

// declares a function with the "C" ABI
extern "C" fn new_i32() -> i32 {
    0
}

// declares a function with the "stdcall" ABI
extern "stdcall" fn new_i32_stdcall() -> i32 {
    0
}

// const functions

// async functions
async fn example(x: &str) -> usize {
    x.len()
}

use std::future::Future;
fn example2<'a>(x: &'a str) -> impl Future<Output = usize> + 'a {
    async move { x.len() }
}

// combining async and unsafe

async unsafe fn unsafe_example(x: *const i32) -> i32 {
    *x
}

async fn safe_example() {
    let p: i32 = 88;
    println!("origin p is {}", p);
    let future = unsafe { unsafe_example(&p) };
    let q = future.await;
    println!("await q is {}", q);
}

// attributes on functions
fn documented() {
    #![doc = "Example"]
    // cfg, cfg_attr, deprecated,doc,export_name,link_section, no_mangle,must_use
}

// allow, warn, deny,forbid

// attributes on function parameters
fn my_len(#[cfg(windows)] slice: &[u16], #[cfg(not(windows))] slice: &[u8]) -> usize {
    slice.len()
}

// Implement

// inherent implementations
pub mod color {
    #[derive(Debug)]
    pub struct Color(pub u8, pub u8, pub u8);
    impl Color {
        pub const WHITE: Color = Color(255, 255, 255);
    }
}

mod values {
    use super::color::Color;
    impl Color {
        pub fn red() -> Color {
            Color(255, 0, 0)
        }
    }
}

// This module is private, meaning that no external crate can access this
// module in the crate may access any publicy visiable item
mod create_helper_module {

    // this function can be used by anything in the current crate
    //
    pub fn create_helper() {}

    // this function can not be used by anything else in the current crate
    // its current module and descendants may access it

    fn implementation_detail() {}
}

// this function is public to the root, meaning that it's availablt to external crates linking
// againtst this one
pub fn public_api() {}

// similary to 'public_api', this function is public to the root, meaning that it's availablt to external crates linking
pub mod submodule {
    use super::create_helper_module;

    pub fn my_method() {
        create_helper_module::create_helper();
    }

    fn my_implementation() {}
}

pub mod outer_mod {
    pub mod inner_mod {
        // visiable within outer_mod
        pub(in crate::outer_mod) fn outer_mod_visiable_fn() {}
        // same the above in 2015
        // pub (in outer_mod) fn outer_visible_fn_2015(){}

        // visable to the entire crate
        pub(crate) fn crate_visible_fn() {}

        // visable within outer_mod
        pub(super) fn super_mod_visible_fn() {
            inner_mod_visible_fn();
        }

        // visalbe within inner_mod
        pub(self) fn inner_mod_visible_fn() {}
    }

    pub fn in_out_fn() {
        inner_mod::outer_mod_visiable_fn();
        inner_mod::crate_visible_fn();
        inner_mod::super_mod_visible_fn();
        // inner_mod::inner_mod_visible_fn()
    }
}

fn outer_out_fn() {
    outer_mod::inner_mod::crate_visible_fn();
    outer_mod::in_out_fn();
}

fn main() {
    // use declarations
    foo(vec![Some(1.0f64), None]);
    let m1 = HashMap::new();
    let m2 = hash_map::HashMap::new();
    bar(m1, m2);

    // use visibility
    quux::bar();
    quux::baz();
    /*
     *
     *     self_fs::file();
     *     std::fs::file();
     */

    let z = Zoo;
    z.zoo();

    ditem!(
        use std as _;
    );

    dstmt!(println!("hello world"));

    // Function
    foo_gen_func2(&[1, 2, 3]);
    let fptr: extern "C" fn() -> i32 = new_i32;
    safe_example();
    println!("my_len is {}", my_len(&[1, 2, 3]));

    // Type alias
    type Point = (u8, u8);
    let pt: Point = (55, 66);
    println!("pt is {:?}", pt);

    struct MyStruct(u32);
    use MyStruct as MyStructAlias;
    // type MyStructType = MyStruct;
    // let my_type = MyStructType(8);
    let s_a = MyStructAlias(8);
    println!("s_a.0 is {:?}", s_a.0);

    // let _ = MyStructType(8); // A type alias to a tuple-struct or unit-struct can not be used to qualify that type's constructor

    // Structs
    struct MyPoint {
        x: i32,
        y: i32,
    }
    let pt2 = MyPoint { x: 10, y: 23 };
    let px: i32 = pt2.x;
    println!("px is {:?}", px);

    struct MyPointNew(i32, i32);
    let pt3 = MyPointNew(10, 33);
    let px1: i32 = match pt3 {
        MyPointNew(x, _) => x,
    };
    println!("pt3 is {:?}, {:?}", pt3.0, pt3.1);
    println!("px1 is {:?}", px1);

    struct Cookie;
    // equivalent to const Cookie: Cookie = Cookie{}
    let c = [Cookie, Cookie {}, Cookie, Cookie {}];

    // Enums
    #[derive(Debug)]
    enum Animal {
        Dog,
        Cat,
    }

    let mut ani: Animal = Animal::Dog;
    ani = Animal::Cat;
    println!("ani is {:?}", ani);

    #[derive(Debug)]
    enum Animal2 {
        Dog(String, f64),
        Cat { name: String, weight: f64 },
    }

    let mut ani2: Animal2 = Animal2::Dog("Cocoa".to_string(), 40.0);
    ani2 = Animal2::Cat {
        name: "Spotty".to_string(),
        weight: 38.2,
    };
    println!("ani2 is {:?}", ani2);

    // the specified discriminant is interpreted as an `isize` value although the compiler is
    // allowed to use a smaller type in the actural memory layout
    enum Enum1 {
        Bar,       //0
        Baz = 123, //123
        Quux,      // 124
    }

    let baz_discriminant = Enum1::Baz as u32;
    println!("baz discriminant is {:?}", baz_discriminant);

    enum ShareDiscriminantError {
        Zero,
        One,
        // OneToo = 1, // collision with previous!\cc
    }

    #[repr(u8)] // max 255
    enum OverflowingDiscriminantError {
        MaxMinusOne = 254,
        Max,
        // MaxPlusOne, // would be 256 but overflows
    }

    // zero-variant enums(Never Type, can not be instantiated and coerced into other types)
    /*
     *
     *     enum ZeroVariants {}
     *     let x: ZeroVariants = panic!();
     *     let y: u32 = x;
     */

    // variant visibility
    macro_rules! mac_variant{
        ($vis: vis $name: ident) => {
            enum $name{
                $vis Unit,
                $vis Tuple(u8, u16),
                $vis Struct{f: u8},
            }
        }
    }

    // Empty `vis` is allowed
    mac_variant! {E}

    //
    #[cfg(FALSE)]
    enum E {
        U,
        T(u8),
        T { f: String },
    }

    #[repr(C)]
    union MyUnion {
        f1: u32,
        f2: u32,
    }
    let u = MyUnion { f1: 1 };
    let f = unsafe { u.f1 };
    println!("union f is {:?}", f); // 1

    // Const
    const BIT1: u32 = 1 << 0;
    const BIT2: u32 = 1 << 1;

    const BITS: [u32; 2] = [BIT1, BIT2];
    const STRING: &'static str = "bitstring";

    #[derive(Debug)]
    struct BitsNStrings<'a> {
        mybits: [u32; 2],
        mystring: &'a str,
    }

    const BITSN_STRINGS: BitsNStrings<'static> = BitsNStrings {
        mybits: BITS,
        mystring: STRING,
    };
    println!("BitsNString is : {:?}", BITSN_STRINGS);

    struct TypeWithDestructor(i32);

    impl Drop for TypeWithDestructor {
        fn drop(&mut self) {
            println!("Dropped. Held {:?}", self.0);
        }
    }

    const ZERO_WITH_DESTRUCTOR: TypeWithDestructor = TypeWithDestructor(0);

    fn create_and_drop_zero_with_destructor() {
        let x = ZERO_WITH_DESTRUCTOR;
    }

    create_and_drop_zero_with_destructor();

    const _: () = {
        struct _SameNameTwice;
    };

    const _: () = {
        struct _SameNameTwice;
    };

    // Statics

    static mut LEVELS: u32 = 0;

    unsafe fn bump_levels_unsafe1() -> u32 {
        let ret = LEVELS;
        LEVELS += 1;
        return ret;
    }

    unsafe {
        let r1 = bump_levels_unsafe1();
        println!("ret is {:?}", r1);
        println!("LEVELS is {:?}", LEVELS);
    }

    /*
     *
     *     use std::sync::atomic;
     *     unsafe fn bump_levels_unsafe2() -> u32 {
     *         return fetch_add(&mut LEVELS, 1);
     *     }
     */

    // Trait

    // generic trait
    trait Seq<T> {
        fn len(&self) -> u32;
        fn elt_at(&self, n: u32) -> T;
        fn iter<F>(&self, f: F)
        where
            F: Fn(T);
    }

    use std::pin::Pin;
    use std::rc::Rc;
    use std::sync::Arc;
    // examples of object safe methods
    trait TraitMethods {
        fn by_ref(self: &Self);
        fn by_ref_mut(self: &mut Self) {}
        fn by_box(self: Box<Self>) {}
        fn by_rc(self: Rc<Self>) {}
        fn by_arc(self: Arc<Self>) {}
        fn by_pin(self: Pin<&Self>) {}
        fn with_lifetime<'a>(self: &'a Self) {}
        fn nested_pin(self: Pin<Arc<Self>>) {}
    }

    // This trait is object safe but methods cannot be dispatched on a trait object
    trait NonDispatchable {
        // Non-methods cannot be dispatched.
        fn foo()
        where
            Self: Sized,
        {
        }

        // Self type is not known until runtime
        fn returns(&self) -> Self
        where
            Self: Sized;

        //`other` may be a diff concrete type of receiver
        fn param(&self, other: Self)
        where
            Self: Sized,
        {
        }

        // generics are not compatible with vtable
        fn typed<T>(&self, x: T)
        where
            Self: Sized,
        {
        }
    }

    struct S;
    impl TraitMethods for S {
        fn by_ref(self: &Self) {}
    }

    impl NonDispatchable for S {
        fn returns(&self) -> Self
        where
            Self: Sized,
        {
            S
        }
    }

    let obj: Box<dyn NonDispatchable> = Box::new(S);
    // obj.returns(); // cann't invoked on a trait object
    // obj.param(S); // cann't invoked on a trait object
    // obj.typed(1); // cann't invoked on a trait object

    let obj2: Box<dyn TraitMethods> = Box::new(S);
    obj2.by_ref();

    // examples of non-object safe trait
    trait NotObjectSelf {
        const CONST: i32 = 1; // associated const
        fn foo() {} // associated function without sized
        fn returns(&self) -> Self; // self in return
        fn typed<T>(&self, x: T) {} // generic type param
        fn nested(self: Rc<Box<Self>>) {}
    }

    impl NotObjectSelf for S {
        fn returns(&self) -> Self {
            S
        }
    }

    // Not able to make a trait object
    // let obj3: Box<dyn NotObjectSelf> = Box::new(S);

    // Self:Size traits are not object-safe
    trait TraiWithSize
    where
        Self: Sized,
    {
    }
    impl TraiWithSize for S {}
    // let obj4: Box<dyn TraiWithSize> = Box::new(S);// cannot made into an object

    // let obj4: Box<dyn TraiWithSize> = Box::new(S); // cannot made into an object

    // Not object safe if `Self` is a type argument
    trait Super<A> {}
    trait WithSelf: Super<Self>
    where
        Self: Sized,
    {
    }
    impl<A> Super<A> for S {}
    impl WithSelf for S {}
    // let obj5: Box<dyn WithSelf> = Box::new(S);

    // Super trait

    trait Shape {
        fn area(&self) -> f64;
    }

    trait Rectangle: Shape {
        fn len(&self) -> f64;
    }

    trait Circle
    where
        Self: Shape,
    {
        fn radius(&self) -> f64 {
            (self.area() / std::f64::consts::PI).sqrt()
        }
    }

    fn print_area_and_radius<C: Circle>(c: C) {
        println!("Area is {:?}", c.area());
        println!("Radius is {:?}", c.radius());
    }

    struct SS;
    impl Shape for SS {
        fn area(&self) -> f64 {
            100.0
        }
    }
    impl Circle for SS {}

    // let circle = Box::new(SS) as Box<dyn Circle>;
    // print_area_and_radius(circle);

    // parameter patterns
    trait T {
        fn f1((a, b): (i32, i32)) {}
        fn f2(_: (i32, i32));
    }

    // item visibility
    macro_rules! create_method{
        ($vis: vis $name: ident $l: literal)=>{
            $vis fn $name(&self) { println!($l)}
        };
    }

    trait T1 {
        create_method! {method_of_t1  "t1"}
    }

    struct SSS;
    impl SSS {
        create_method! {pub method_of_s "s"}
    }

    impl T1 for SSS {}

    let s = SSS;
    s.method_of_t1();
    s.method_of_s();

    use self::color::Color;
    println!("White color is {:?}", color::Color::WHITE);
    color::Color::red();
    Color::red();

    #[derive(Copy, Clone)]
    struct PointC {
        x: f64,
        y: f64,
    }

    struct MyCircle {
        radius: f64,
        center: PointC,
    }

    struct BoundingBox {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    }

    trait MyShape {
        fn draw(&self) {
            println!("do draw in myShape");
        }
        fn bounding_box(&self) -> BoundingBox;
    }

    impl Copy for MyCircle {}

    impl Clone for MyCircle {
        fn clone(&self) -> MyCircle {
            *self
        }
    }

    impl MyShape for MyCircle {
        fn bounding_box(&self) -> BoundingBox {
            let r = self.radius;

            BoundingBox {
                x: self.center.x - r,
                y: self.center.y - r,
                w: 2.0 * r,
                h: 2.0 * r,
            }
        }
    }

    let mc = MyCircle {
        radius: 50.0,
        center: PointC { x: 20.0, y: 10.0 },
    };
    mc.draw();
    let my: BoundingBox = mc.bounding_box();
    println!("box x,y,w,h = {}, {}, {}, {}", my.x, my.y, my.w, my.h);

    // impl<T> Seq<T> for Vec<T> {}
    // impl Seq<bool> for u32 {}
    // Treat the int as a sequence of bits

    // unsafe extern "abi" for<'11,...,'lm> fn (A1,...,An) ->R

    /*
     *     // variadic functions
     *     extern "C" {
     *         fn foo4(x: i32, ...);
     *     }
     *
     */
    // link attribute
    //
    #[link(name = "crypto")]
    extern "C" {}
    /*
     *
     *     #[link(name = "CoreFoundation", kind = "framework")]
     *     extern "C" {}
     *
     */
    #[link(wasm_import_module = "foo")]
    extern "C" {}

    // link name attribute

    extern "C" {
        #[link_name = "actual symbol_name"]
        fn name_in_rust();
    }

    // type and lifetime parameter
    fn foo2<'a, T>() {}
    trait A2<U> {}
    struct Ref2<'a, T>
    where
        T: 'a,
    {
        r: &'a T,
    }

    // attributes
    /*
     *
     *     #[derive(MyFlexibleClone)]
     *     struct Foo2<#[my_flexible_clone(unbounded)] H> {
     *         a: *const H,
     *     }
     */

    // Associated items

    struct S1 {
        field: i32,
    }

    impl S1 {
        fn new() -> S1 {
            S1 { field: 0i32 }
        }
    }

    let _s = S1::new();
    println!("s field is {}", _s.field);

    trait Num {
        fn from_i32(n: i32) -> Self;
    }

    impl Num for f64 {
        fn from_i32(n: i32) -> f64 {
            n as f64
        }
    }

    let _: f64 = Num::from_i32(42);
    let _: f64 = <_ as Num>::from_i32(42);
    let _: f64 = <f64 as Num>::from_i32(42);
    let _: f64 = f64::from_i32(42);

    // method
    struct S3;
    type Alias = S3;
    trait T3 {
        type Output;
    }

    impl T3 for S3 {
        type Output = S3;
    }

    impl S3 {
        fn by_value(self: Self) {}
        fn by_ref(self: &Self) {}
        fn by_mut_ref(self: &mut Self) {}
        fn by_box(self: Box<Self>) {}
        fn by_rc(self: Rc<Self>) {}
        fn by_arc(self: Arc<Self>) {}
        fn by_pin(self: Pin<&Self>) {}
        fn expicit_type(self: Arc<S3>) {}
        fn with_lifetime<'a>(self: &'a Self) {}
        fn nested<'a>(self: &mut &'a Arc<Rc<Box<Alias>>>) {}

        fn via_projection(self: <S3 as T3>::Output) {}
    }

    trait Changer: Sized {
        fn change(mut self) {}
        fn modify(mut self: Box<Self>) {}
    }

    trait AssociatedType {
        type Assoc;
    }

    struct S4;
    struct S5;
    impl AssociatedType for S4 {
        // definination
        type Assoc = S5;
    }

    impl S5 {
        fn new() -> S5 {
            S5
        }
    }

    let _s5: S5 = <S4 as AssociatedType>::Assoc::new();

    trait Container {
        type E;
        fn empty() -> Self;
        fn insert(&mut self, elem: Self::E);
    }

    impl<T> Container for Vec<T> {
        type E = T;
        fn empty() -> Vec<T> {
            Vec::new()
        }

        fn insert(&mut self, x: T) {
            self.push(x);
        }
    }

    trait ConstantId {
        const ID: i32;
    }

    struct S7;

    impl ConstantId for S7 {
        const ID: i32 = 1;
    }

    trait ConstIdDefault {
        const ID: i32 = 4;
    }

    struct S8;

    // impl ConstIdDefault for S7;

    impl ConstIdDefault for S8 {
        const ID: i32 = 8;
    }

    println!("const id is for S7 {}", S7::ID);
    println!("const id for S8 is {}", S8::ID);

    // visibility and privacy: crate, super, self,
    outer_out_fn();
}
