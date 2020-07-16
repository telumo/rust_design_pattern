mod adapter;
mod factory_method;
mod iterator;
mod prototype;
mod singleton;
mod template_method;

fn main() {
    iterator::run();
    adapter::run();
    template_method::run();
    factory_method::run();
    singleton::run();
    prototype::run();
}
