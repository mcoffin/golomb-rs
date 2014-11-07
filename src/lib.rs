use std::io;

pub use bits::{BitWriter, BitBuffer};

mod bits;

pub struct Encoder<T: BitWriter> {
    pub out: T,
}

fn bit_len(mut a: uint) -> uint {
    // TODO: optimize
    let mut b = 0u;
    while a > 0 {
        a >>= 1;
        b += 1;
    }
    b
}

impl<T: io::Writer> Encoder<BitBuffer<T>> {
    pub fn new(out: T) -> Encoder<BitBuffer<T>> {
        Encoder {
            out: BitBuffer::new(out),
        }
    }
}

impl<T: BitWriter> Encoder<T> {
    /// Generic write function for anything that can be translated into a uint
    pub fn write<A>(&mut self, a: A, strategy: |A| -> uint) -> io::IoResult<()> {
        self.write_uint(strategy(a))
    }

    /// Writes a uint to the encoder
    pub fn write_uint(&mut self, a: uint) -> io::IoResult<()> {
        let size = bit_len(a);
        try!(self.out.write_bits(0x0, size - 1));
        self.out.write_bits(a, size)
    }

    /// Flushes the Encoder
    pub fn flush(&mut self) -> io::IoResult<()> {
        self.out.flush()
    }
}
