// macro can be invoked in the following  situations
// use lazy_static::lazy_static; // Path-based import, first look up in textual scoping

use std::any::type_name;
use std::cell::RefCell;

fn main() {
    // expression
    let x = vec![1, 2, 4];

    // statement
    println!("x[0] is {:?}", x[0]);

    // pattern
    macro_rules! pat {
        ($i: ident) => {
            Some($i)
        };
    }

    // let y: i32 = 4;
    if let pat!(x) = Some(1) {
        println!("Got Some(1)");
    } else {
        println!("Not Got Some(1)");
    }

    // type
    macro_rules! Tuple{
        {$A:ty, $B:ty } => {($A, $B)};
    }

    fn type_of<T>(_: &T) -> &'static str {
        type_name::<T>()
    }

    type N2 = Tuple!(i32, i32);
    let aa: N2 = (3, 4);
    println!("aa value is {:?}, {:?}", type_of(&aa.0), type_of(&aa.1));

    // item
    thread_local!(static FOO:RefCell<u32> = RefCell::new(1));

    // associated item
    macro_rules! const_makder {
        ($t:ty, $v:tt) => {
            const CONST: $t = $v;
        };
    }

    trait T {
        const_makder! {i32, 7}
    }

    // macro calls within macros
    macro_rules! example {
        () => {
            println!("Macro call in a macro")
        };
    }

    example!();

    // transcribing

    macro_rules! ambiguity {
        ($($i: ident)* $j:ident) => {};
    }

    // ambiguity!(error);
    /*
     *
     *     macro_rules! foo {
     *         ($l:expr) => {
     *             bar!($l);
     *         };
     *     }
     */

    macro_rules! foo2 {
        ($l:tt) => {
            bar!($l);
        };
    }

    macro_rules! bar {
        (3) => {
            println!("Got 3");
        };
    }

    // foo!(3);
    // foo!(4);

    foo2!(3);
    //foo2!(4);

    // textual definition
    macro_rules! lazy_static {
        (lazy) => {
            println!("Got self textual ")
        };
    }

    lazy_static!(lazy); // textual lookup
                        // self::lazy_static! {} //path-based lookup

    mod has_macro {
        // m! {} // Error: m is not in scope

        macro_rules! m {
            () => {};
        }

        m! {} // OK: appears after declaration of m
    }

    // m! {} // Error: m is not in scope

    #[macro_use] // make mod scope not end when mod is closed
    mod inner {
        macro_rules! n {
            () => {};
        }
    }
    n!();

    // procedural macros

    // function-like macros
    // derive macros
    // attribute macros
}
