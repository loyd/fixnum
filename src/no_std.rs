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

#[cfg(any(test, feature = "std"))]
fn f64_significant_digits_std(value: f64) -> usize {
    (value.abs().log10() + 1f64).floor() as usize
}

#[cfg(any(test, not(feature = "std")))]
fn f64_significant_digits_no_std(value: f64) -> usize {
    let mut abs_value = if value.is_sign_negative() {
        -value
    } else {
        value
    };

    let mut significant = 0;
    while abs_value >= 1f64 {
        significant += 1;
        abs_value /= 10f64;
    }

    significant
}

pub(crate) fn f64_significant_digits(value: f64) -> usize {
    #[cfg(feature = "std")]
    return f64_significant_digits_std(value);

    #[cfg(not(feature = "std"))]
    return f64_significant_digits_no_std(value);
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

    #[test]
    fn compare_f64_significant_digits_std_and_no_std_output() {
        let values = [-100., -55.14, -1., -0.5, -0., 0., 0.5, 1., 55.14, 100.];

        for value in values {
            let no_std_out = f64_significant_digits_no_std(value);
            let std_out = f64_significant_digits_std(value);
            assert_eq!(no_std_out, std_out, "mismatch for {}", value);
        }
    }
}
