mod ownership;
mod structrue;
mod collector;
mod generics;
mod errorhandler;

use clap::Parser;
#[derive(Parser)]
struct Cli {
    #[arg(short, long = "pattern")]
    pattern: String,
}

fn main() {
    let args = Cli::parse();

    match args.pattern.as_str() {
        "ownership" => {
            /* 所有权学习 */
            ownership::base::ownership();
            ownership::base::function_ownership();
            ownership::base::return_val_ownership();
            ownership::reference::func_arg_reference();
            ownership::slice::slice_function();
        },
        "structrue" => {
            /* 结构体 */
            structrue::base::print_struct();
            structrue::rectangles::calculate_rect();
        },
        "collector" => {
            /* 容器 */
            collector::vectorcollector::print_vector_collector();
            collector::hashmapcollector::print_hashmap_collector();
        },
        "generics" => {
            /* 泛型 */
            generics::generics::print_generics_info();
        },
        "errorhandler" => {
            /* 错误处理 */
            errorhandler::resulthandler::result_handler_print();
        },
        _ => {
            panic!("Not pattern");
        },
    }

}
