pub fn r_and_b(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    change(&mut s);
    println!("The length of '{}' is {}.",s,s.len());

    /* WRONG!
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM (cannot borrow `s` as mutable because it is also borrowed as immutable)
    println!(" {s}, {r1}, {r2}");*/

    /* CORRECT!
    Note that a reference’s scope starts from where it is introduced and continues 
    through the last time that reference is used. For instance, this code will 
    compile because the last usage of the immutable references is in the println!, 
    before the mutable reference is introduced:
     */
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

  // Mutable References
fn change(some_string: &mut String) { 
    some_string.push_str(", world");
}