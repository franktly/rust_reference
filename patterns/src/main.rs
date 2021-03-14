fn main() {
    // Literal Pattern

    struct Person {
        name: &'static str,
        car: &'static str,
        age: u32,
    }

    let person = Person {
        car: "Tesla",
        age: 33,
        name: "tly",
    };

    if let Person {
        car: _,
        age: person_age @ 13..=19,
        name: ref person_name,
        ..
    } = person
    {
        println!("{} has a car and is {} years old", person_name, person_age);
    }

    let (x, y) = (1, 2);

    if let (a, 3) = (1, 2) {
        panic!("shouldn't reach here");
    } else if let (a, 4) = (3, 4) {
        println!("Matched({},4)", a);
    }

    for i in -2..=5 {
        match i {
            -1 => println!("minus one"),
            1 => println!("one"),
            2 | 4 => println!("either two or four"),
            _ => println!("mathed none of the arms"),
        }
    }
    // Identifier Pattern
    let x = 2;
    match x {
        e @ 1..=5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    // Wildcard Pattern
    /*
     *
     *     match slice {
     *         [] => println!("slice is empty"),
     *         [one] => println!("single element {}", one),
     *         [head, tail @ ..] => println!("head = {}, tail = {:?}", head, tail),
     *     }
     */

    let slice = ["a", "b", "!"];
    match slice {
        [.., "!"] => println!("!!!"),
        [start @ .., "z"] => println!("starts with {:?}", start,),
        ["a", end @ ..] => println!("starts with {:?}", end,),
        rest => println!("{:?}", rest),
    }

    let slice = ["a", "b", "c", "d"];
    if let [.., last_but_one, _] = slice {
        println!("next to the last is {}", last_but_one);
    }

    let tuple = (1, 2, 3, 4, 5);
    match tuple {
        (1, .., y, z) => println!("y= {}, z= {}", y, z),
        (.., 5) => println!("tail must 5"),
        (..) => println!("matched everything else"),
    }

    // Rest Pattern

    let c = 'B';
    let valid_variable = match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        _ => false,
    };
    println!("valid var is {:?}", valid_variable);

    // Reference Pattern
    let int_ref = &3;
    let a = match *int_ref {
        0 => "zero",
        _ => "some",
    };

    let b = match int_ref {
        &0 => "zero",
        &3 => "three",
        _ => "some",
    };
    println!("a is {:?}, b is {:?}", a, b);

    // Struct Pattern

    struct Point {
        x: u32,
        y: u32,
    };

    let s = Point { x: 10, y: 3 };

    match s {
        Point { x: 10, y: 20 } => println!("p1"),
        Point { y: 10, x: 20 } => println!("p2"),
        Point { x: 10, .. } => println!("p3"),
        Point { .. } => println!("else"),
    }

    // TupleStruct Pattern
    struct Point2(u32, u32);
    let t = Point2(10, 30);
    match t {
        Point2 { 0: 10, 1: 20 } => println!("p1"),
        Point2 { 1: 10, 0: 20 } => println!("p2"),
        Point2 { 0: 10, .. } => println!("p3"),
        Point2 { .. } => println!("else"),
    }

    struct P3 {
        a: u32,
        b: char,
        c: &'static str,
    };

    let mut SP = P3 {
        a: 10,
        b: 'X',
        c: "false",
    };

    match SP {
        P3 {
            a: 10,
            b: 'X',
            c: "false",
        } => println!("match c false"),
        P3 {
            a: 10,
            b: 'X',
            ref c,
        } => println!("match c ref"),
        P3 {
            a: 10,
            b: 'X',
            ref mut c,
        } => println!("match c mut ref"),
        P3 {
            a: 10,
            b: 'X',
            c: _,
        } => println!("match c anything "),
        P3 { a: _, b: _, c: _ } => println!("match a , b c anything",),
    }
    // Tuple Pattern

    let pair = (10, "ten");
    let (a, b) = pair;
    println!("a is {:?}, b is {:?}", a, b);
    // Grouped Pattern

    // Enclosing a pttern in parentheses can be used to explicitly control the precedence of
    // compound patterns. For example a reference pattern next to a range pattern
    let int_ref = &3;
    match int_ref {
        &(0..=5) => println!("matched int ref grouped pattern"),
        _ => println!("anything"),
    }
    // Slice Pattern
    // slice pattern  can match both arrays of fixed size and slice of dynamic size
    let arr = [1, 2, 3];
    match arr {
        [1, _, _] => println!("starts with one"),
        [a, b, c] => println!("starts with anything else : {:?}, {:?}, {:?}", a, b, c),
    };

    let v = vec![1, 2, 3];
    match v[..] {
        [a, b] => println!("numbers with two {:?}, {:?}", a, b),
        [a, b, c] => println!("numbers with three {:?}, {:?}, {:?}", a, b, c),
        _ => println!("numbers with anything else"),
    };

    //Path Pattern
}
