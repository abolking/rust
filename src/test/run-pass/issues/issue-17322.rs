// run-pass
// pretty-expanded FIXME #23616

#![feature(box_syntax)]

use std::io::{self, Write};

fn f(wr: &mut Write) {
    wr.write_all(b"hello").ok().expect("failed");
}

fn main() {
    let mut wr = box io::stdout() as Box<Write>;
    f(&mut wr);
}
