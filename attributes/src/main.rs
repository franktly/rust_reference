// #![crate_type = "lib"]
//
// #![recursion_limit = "3"]
#![type_length_limit = "8"]

#[test]
fn fn1() {}

#[cfg(target_os = "linux")]
mod mod1 {}

#[allow(non_camel_case_types)]
type int8_t = i8;

fn fn2() {
    // inner attributation
    #![allow(unused_variables)]
    let x = ();
    let y = ();
}

// tool attributation

#[rustfmt::skip]
struct S{}

#[clippy::cyclomatic_complexity = "10"]
pub fn fn3() {}

// meta item attribute syntax:
//MetaWord, MetaNameValueStr, MetaListPaths, MetaListIdents, MetaListNameValueStr
//
//built-in attributaion index
//
//Conditional compilation:
//
//cfg, cfg_attr
//
//Testing:
//
//test, ignore, should_panic
//
//Derive:
//
//derive, automatically_derived
//
//Macros:
//
//macro_export, marco_use, proc_macro, proc_macro_derive
//
//Diagnostics:
//
//allow, warn,deny,forbid,deprecated, must_use
//
//ABI,Linking,Symbols and FFI:
//
//link, link_name,no_link,repr, crate_type, no_main,export_name, link_section, no_mangle, used,
//crate_name
//
//Code Gen:
//
//inline, cold, no_builtins, target_features, track_caller
//
//Documentation:
//
//doc
//
//Preludes:
//
//no_std, no_implicit_prelue
//
//Modules:
//
//path
//
//Limits:
//
//recursion_limit,type_length_limit
//
//Runtimes:
//
//panic_handler,global_allocator, windows_subsystem
//
//Features:
//
//feature
//
//Type System:
//
//no_exhausitive
//
//

fn main() {
    // Test
    #[test]
    #[ignore = "not yet implemented"]
    fn fn1() {}

    #[test]
    #[should_panic = "value don't match"]
    fn my_test() {
        assert_eq!(1, 2, "value don't match");
    }

    // Derive

    #[derive(PartialEq, Clone)]
    struct S1<T> {
        a: i32,
        b: T,
    }

    // Disgnostic

    #[warn(missing_docs)]
    pub mod m1 {

        #[allow(missing_docs)]
        pub mod nested {
            #[allow(missing_docs)]
            pub fn undocumented_one() -> i32 {
                1
            }

            #[warn(missing_docs)]
            pub fn undocumented_two() -> i32 {
                1
            }
        }

        #[deny(missing_docs)]
        pub fn undocumented_three() -> i32 {
            1
        }

        #[forbid(missing_docs)]
        pub fn undocumented_four() -> i32 {
            1
        }
    }

    // lint
    /*
     *
     *     #![warn(clippy::pedantic)]
     *     #![warn(clippy::filter_map)]
     *     #![warn(clippy::cmp_nan)]
     *     fn fn2(){}
     */

    #[deprecated(since = "5.2", note = "wrong and deprecated")]
    pub fn fn3() {}

    #[must_use]
    struct MustUse {}

    // none violate unused_must_use lint

    #[must_use]
    fn fn4() {
        println!("must used")
    }

    // violate unused_must_use lint
    fn4();

    #[must_use]
    trait T1 {}
    impl T1 for i32 {}
    fn get_t() -> impl T1 {
        8i32
    }

    // violate unused_must_use lint
    get_t();

    trait T2 {
        #[must_use]
        fn use_me(&self) -> i32;
    }

    impl T2 for i32 {
        fn use_me(&self) -> i32 {
            0i32
        }
    }

    // violate unused_must_use lint
    4.use_me();

    fn f5() {}

    // none violate unused_must_use lint
    (f5(),);
    Some(f5());

    if true {
        f5()
    };

    match true {
        _ => f5(),
    }

    // Code Generation

    #[track_caller]
    fn f6() {
        println!("{}", std::panic::Location::caller());
    }

    #[track_caller]
    fn call_f6() {
        f6();
    }
    call_f6();

    fn call_f6_2() {
        call_f6();
    }

    call_f6_2();

    // recursion_limit applied to the crate level
    // defalult to 128
    macro_rules! a {
        () => {
            a!(1)
        };
        (1) => {
            a!(2)
        };
        (2) => {
            a!(3)
        };
        (3) => {
            a!(4)
        };
        (4) => {};
    }

    a!();

    // type_length_limit limits the max num of type substitutions
    // default to 1048576

    fn f7<T>(x: T) {}
    f7((1, 2, 3, 4, 5, 6, 7, 8, 9));

    // Type system

    #[non_exhaustive]
    pub struct S2 {
        pub w: u8,
        pub h: u8,
    }

    #[non_exhaustive]
    pub enum E1 {
        Message(String),
        Other,
    }

    pub enum E2 {
        #[non_exhaustive]
        Send { from: u32, to: u32 },

        #[non_exhaustive]
        Reaction(u32),

        #[non_exhaustive]
        Quit,
    }

    let s2 = S2 { w: 8, h: 8 };
    let e1 = E1::Other;
    let e2 = E2::Reaction(3);
}

//
