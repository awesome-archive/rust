// build-pass

//! Like drop-tracking-parent-expression, but also tests that this doesn't ICE when building MIR
#![feature(coroutines)]

fn assert_send<T: Send>(_thing: T) {}

#[derive(Default)]
pub struct Client { pub nickname: String }

fn main() {
    let g = move || match drop(Client { ..Client::default() }) {
        _status => yield,
    };
    assert_send(g);
}
