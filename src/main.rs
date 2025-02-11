mod ownership;
mod structrue;
mod collector;

fn main() {
    /* 所有权学习 */
    ownership::base::ownership();
    ownership::base::function_ownership();
    ownership::base::return_val_ownership();

    ownership::reference::func_arg_reference();

    ownership::slice::slice_function();

    /* 结构体 */
    structrue::base::print_struct();
    structrue::rectangles::calculate_rect();

    /* 容器 */
    collector::vectorcollector::print_vector_collector();
    collector::hashmapcollector::print_hashmap_collector();
}
