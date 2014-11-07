use std::io;

/// A BitWriter can write data at the bit level
pub trait BitWriter {
    fn write_bits(&mut self, data: uint, count: uint) -> io::IoResult<()>;
    fn flush(&mut self) -> io::IoResult<()>;
}

pub struct BitBuffer<T: io::Writer> {
    out: T,
    current_byte: u8,
    remaining_bits: u8,
}

impl<T: io::Writer> BitBuffer<T> {
    /// Wraps an existing Writer in a BitBuffer
    pub fn new(out: T) -> BitBuffer<T> {
        BitBuffer {
            out: out,
            current_byte: 0,
            remaining_bits: 8,
        }
    }

    /// Clears the buffer
    pub fn clear(&mut self) {
        self.current_byte = 0;
        self.remaining_bits = 8;
    }
}

impl<T: io::Writer> BitWriter for BitBuffer<T> {
    fn write_bits(&mut self, value: uint, mut count: uint) -> io::IoResult<()> {
        if count < self.remaining_bits as uint {
            let mask = value as u8;
            self.current_byte <<= count;
            let shift = 8 - count;
            self.current_byte |= ((mask << shift) >> shift) as u8;
            self.remaining_bits -= count as u8;
            Ok(())
        } else {
            let mask = (value >> (count - (self.remaining_bits as uint))) as u8;
            self.current_byte <<= self.remaining_bits as uint;
            self.current_byte |= mask;
            count -= self.remaining_bits as uint;
            self.remaining_bits = 0;

            try!(self.flush());

            self.write_bits(value, count)
        }
    }

    /// Flushes the buffer, then clears it
    fn flush(&mut self) -> io::IoResult<()> {
        if self.remaining_bits == 8 {
            return Ok(())
        }

        self.current_byte <<= self.remaining_bits as uint;

        try!(self.out.write_u8(self.current_byte));

        self.clear();
        Ok(())
    }
}
