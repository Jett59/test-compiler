use std::error::Error;

use inkwell::context::Context;

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("Test");
    let function = module.add_function("main", context.i32_type().fn_type(&[], false), None);
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    let i32_type = context.i32_type();
    let i32_zero = i32_type.const_int(0, false);
    let return_value = i32_zero;
    builder.build_return(Some(&return_value))?;
    module.print_to_stderr();
    Ok(())
}
