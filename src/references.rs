// Reference pointers - Points to a resource in memory

pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Array {:?}", (arr1, arr2));

    /* With non-primitives, if we want to assign another variable to a piece of data,
    the first variable will no longer hold that value so we will need to use a reference
    to point to that data*/

    // Vector
    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}