#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
mod generatic;
mod iter_stream;
mod log_env;
mod minitrace_cli;
mod quick_xml;
mod sort_merge_join;
mod stream_flatmap; 
mod trait_fn;
mod plan_visitor;
fn main() {
    println!("Hello, world!");
}
