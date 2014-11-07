extern crate golomb;

use std::os;
use std::io;
use std::collections;

fn main() {
    let args = os::args();
    assert!(args.len() >= 2);

    let (tx, rx) = channel();

    let mut out = golomb::Encoder::new(io::ChanWriter::new(tx));

    spawn(proc() {
        let mut w = io::MemWriter::new();
        for refs in rx.iter() {
            let mut bytes = refs.iter().map(|&q| q);
            for b in bytes {
                print!("{:08t} ", b);
                w.write_u8(b).ok().expect("buffer sizing error");
            }
        }
        println!("");

        let bytes_ref = w.get_ref();
        println!("Compressed to {} bytes", bytes_ref.len());
        let v = collections::bitv::from_bytes(bytes_ref);
        let mut int_iter = golomb::Decoder::new(v.iter()).map(|u| -> int {
            if (u & 0x01) != 0 {
                ((u as int) - 1) / -2
            } else {
                (u as int) / 2
            }
        });
        for i in int_iter {
            print!("{:d} ", i);
        }
        println!("");
    });

    for arg in args.slice(1, args.len()).iter() {
        let x_opt: Option<int> = from_str(arg.as_slice().trim());
        let x = x_opt.unwrap();

        out.write(x, |u| {
            if u >= 0 {
                2 * u as uint
            } else {
                (-(2 * u) + 1) as uint
            }
        }).ok().expect("failed to write");
    }
    println!("Compressing {} bytes", (args.len()-1) * std::mem::size_of::<int>());
    out.flush().ok().expect("failed to flush");
}
