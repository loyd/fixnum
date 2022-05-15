//! Contains logic related to no_std functionaligy

use core::fmt;

/// Avoids the use of [`std::io::Cursor`] in `no_std` context
pub(crate) struct Cursor<T> {
    buffer: T,
    position: usize,
}
impl<T> Cursor<T> {
    pub(crate) fn new(buffer: T) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }
    pub(crate) fn position(&self) -> usize {
        self.position
    }
}
impl<T: AsMut<[u8]>> fmt::Write for Cursor<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let raw_s = s.as_bytes();
        let pos = self.position();

        let buf = &mut self.buffer.as_mut()[pos..pos + raw_s.len()];

        if raw_s.len() > buf.len() {
            return Err(fmt::Error);
        }

        buf.copy_from_slice(raw_s);
        self.position += raw_s.len();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Write;

    use super::*;

    #[test]
    fn cursor_over_bytes() {
        let mut cursor = Cursor::new([0u8; 64]);
        write!(&mut cursor, "{:.*}", 5, 12.12345f64).unwrap();
        assert_eq!(cursor.position(), 8);
    }
}
