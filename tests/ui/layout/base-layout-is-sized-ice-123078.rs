// ICE !base.layout().is_sized()
// issue: rust-lang/rust#123078

struct S {
    a: [u8],
    //~^ ERROR the size for values of type `[u8]` cannot be known at compilation time
    b: (),
}

const C: S = unsafe { std::mem::transmute(()) };
const _: [(); {
    C;
    0
}] = [];

pub fn main() {}
