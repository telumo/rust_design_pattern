mod adapter;
mod factory_method;
mod iterator;
mod template_method;

fn main() {
    iterator::run();
    adapter::run();
    template_method::run();
    factory_method::run();
}
