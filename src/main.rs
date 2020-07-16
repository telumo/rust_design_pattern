mod iterator;
mod adapter;
mod template_method;
mod factory_method;

fn main() {
    iterator::run();
    adapter::run();
    template_method::run();
    factory_method::run();
}