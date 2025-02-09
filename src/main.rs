mod ownership;

fn main() {
    /* 所有权学习 */
    ownership::base::ownership();
    ownership::base::function_ownership();
    ownership::base::return_val_ownership();

    ownership::reference::func_arg_reference();
    
    ownership::slice::slice_function();
}
