struct Buffer<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'b, 'a: 'b> Buffer<'a> {
    fn new(b: &'a [u8]) -> Buffer {
        Buffer { buf: b, pos: 0 }
    }

    fn read_bytes_early(&'b mut self) -> &'a [u8] {
        self.pos += 3;
        &self.buf[self.pos - 3..self.pos]
    }
    fn read_bytes_late<'c>(&'c mut self) -> &'c [u8] {
        self.pos += 3;
        &self.buf[self.pos - 3..self.pos]
    }
}
fn test<'x>() {
    // let bf = Buffer::read_bytes_early as fn(&'static mut Buffer::<'static>)-> &'static [u8];
    // let bg = Buffer::read_bytes_late as for<'a> fn(&'a mut Buffer::<'x>)-> &'a [u8];

    let bf = Buffer::read_bytes_early; // as fn(&'static mut Buffer::<'static>)-> &'static [u8];
    let bg = Buffer::read_bytes_late; // as for<'a> fn(&'a mut Buffer::<'x>)-> &'a [u8];
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_early_late_bound() {
        test()
    }
}
