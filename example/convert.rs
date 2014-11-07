extern crate golomb;

use std::os;
use std::io;

fn main() {
    let args = os::args();
    assert!(args.len() >= 2);

    let (tx, rx) = channel();

    let mut out = golomb::Encoder::new(io::ChanWriter::new(tx));

    spawn(proc() {
        for refs in rx.iter() {
            let mut bytes = refs.iter().map(|&q| q);
            for b in bytes {
                print!("{:08t}", b);
            }
        }
        println!("");
    });

    for arg in args.slice(1, args.len()).iter() {
        let x_opt: Option<uint> = from_str(arg.as_slice().trim());
        let x = x_opt.unwrap();

        out.write_uint(x).ok().expect("failed to write");
    }
    out.flush().ok().expect("failed to flush");
}
