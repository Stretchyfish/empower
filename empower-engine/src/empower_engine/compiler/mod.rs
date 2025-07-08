pub mod integer;
pub use integer::execute_integer_node;
pub use integer::execute_debug_integer_node;

pub mod addition;
pub use addition::execute_debug_addition_node;

pub mod print;
pub use print::execute_debug_print_node;