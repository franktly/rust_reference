extern crate proc_macros_examp;
use proc_macros_examp::make_answer;
use proc_macros_examp::return_as_is;
use proc_macros_examp::show_streams;
use proc_macros_examp::HelperAttr;
use proc_macros_examp::NewAnswerFn;

make_answer!();

#[derive(NewAnswerFn)]
struct S;

#[derive(HelperAttr)]
struct S2 {
    #[helper]
    filed: (),
}

#[return_as_is]
fn invoke1_1() {}

#[show_streams]
fn invoke1() {}

#[show_streams(bar)]
fn invoke2() {}

#[show_streams(multiple => tokens)]
fn invoke3() {}

#[show_streams{delimiters}]
fn invoke4() {}

fn main() {
    println!("answer is {:?}", answer());
    println!("answer is {:?}", new_answer());
    invoke1_1();
    invoke1();
    invoke2();
    invoke3();
    invoke4();
    // let s1 : S  = new S();
    // let s2 : S2 = new S2();
}
