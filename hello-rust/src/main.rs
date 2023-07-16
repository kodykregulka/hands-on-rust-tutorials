
use std::fmt;
use std::mem;
//use rand::prelude::*;

// An attribute to hide warnings for unused code.
#[allow(dead_code)]

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    return (bool_param, int_param)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
impl fmt::Display for Matrix {
    fn fmt(&self, f:  &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix:Matrix) -> Matrix {
    return Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn tuple_tutorial(){
    //let x: u8 = random();
    //println!("Hello, world! {}", x);
    let x= 5 + 5;
    println!("hello world {}", x + 3i64);

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                                            -1i8, -2i16, -3i32, -4i64,
                                            0.1f32, 0.2f64,
                                        'a', true);

    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple first value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8,), -2i16);
    println!("tuple_of_tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);

    println!("{}", transpose(matrix));
}

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn array_tutorial(){
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1,2,3,4,5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", ys[0]);
    println!("Second element of the array: {}", ys[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// a unit struct
struct Unit;

//tutple struct
struct Pair(i32, f32);

//struct with two fields
#[derive(Copy, Clone)]
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

//struct within struct
#[derive(Debug)]
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect:Rectangle) -> f32 {
    let Rectangle{top_left, bottom_right} = rect;
    return ((top_left.x - bottom_right.x) * (top_left.y - bottom_right.y)).abs()
}

fn square(point:Point, length:f32) -> Rectangle{
    return Rectangle{top_left: point, bottom_right: Point{x: point.x+length, y: point.y+length}};
}

fn struct_tutorials(){
    //create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age};

    println!("{:?}", peter);

    let point = Point {x: 10.3, y: 0.4};
    println!("point coordinates: {}, {}", point.x, point.y);
    
    //make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point { x: 5.2, ..point};

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;
    let _rectangle = Rectangle {
        //struct instantiation is an expression too
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    //instaniate a unit struct
    let _unit = Unit;

    //instaniate a tuple struct
    let pair = Pair(1,0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rect area: {}", rect_area(Rectangle { top_left: Point{x: 3.1, y: 4.3}, bottom_right: Point{x: 4.3,y: 6.1} }));

    let _square = square(Point { x: 3.2, y: 8.1 }, 10.0);
    println!("{:?}", _square);
    println!("{:?}", rect_area(_square));
}

fn main() {
    //tuple_tutorial()
    //array_tutorial();
    struct_tutorials();
}
