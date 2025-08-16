//mod variables_and_mutability;
//mod functions;
mod control_flow;

fn main() {
    //variables_and_mutability::variables_and_mutability();   
    //functions::test_function(5,"cinco"); 
    //println!("{} and {}", functions::five(),functions::six());
    println!("The statement is: {}",control_flow::is_equal(5, 6));
    control_flow::loop_counter();
    control_flow::while_counter();
    control_flow::for_fn();
}