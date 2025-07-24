pub mod compile_integer;
pub use compile_integer::execute_integer_node;
pub use compile_integer::execute_debug_integer_node;

pub mod compile_addition;
pub use compile_addition::execute_debug_addition_node;

pub mod compile_number;
pub use compile_number::execute_debug_number_node;

pub mod compile_print;
pub use compile_print::execute_debug_print_node;

pub mod compile_multiply;
pub use compile_multiply::execute_debug_multiply_node;