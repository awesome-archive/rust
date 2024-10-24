#![feature(const_trait_impl, effects)] //~ WARN the feature `effects` is incomplete

const fn foo() {}

#[const_trait]
trait Bar {
    fn bar();
}

impl Bar for () {
    fn bar() {}
}

fn main() {
    foo::<true>();
    //~^ ERROR: function takes 0 generic arguments but 1 generic argument was supplied
    <() as Bar<true>>::bar();
    //~^ ERROR: trait takes 0 generic arguments but 1 generic argument was supplied
}

const FOO: () = {
    foo::<false>();
    //~^ ERROR: function takes 0 generic arguments but 1 generic argument was supplied
    <() as Bar<false>>::bar();
    //~^ ERROR: trait takes 0 generic arguments but 1 generic argument was supplied
};
