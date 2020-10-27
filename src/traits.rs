use crate::PathValue;
use byteorder::{BigEndian, WriteBytesExt};

/// General trait for an HDPath.
/// Common implementations are [`StandardHDPath`], [`AccountHDPath`] and [`CustomHDPath`]
///
/// [`StandardHDPath`]: struct.StandardHDPath.html
/// [`AccountHDPath`]: struct.AccountHDPath.html
/// [`CustomHDPath`]: struct.CustomHDPath.html
pub trait HDPath {

    /// Size of the HD Path
    fn len(&self) -> u8;

    /// Get element as the specified position.
    /// The implementation must return `Some<PathValue>` for all values up to `len()`.
    /// And return `None` if the position if out of bounds.
    ///
    /// See [`PathValue`](enum.PathValue.html)
    fn get(&self, pos: u8) -> Option<PathValue>;

    /// Encode as bytes, where first byte is number of elements in path (always 5 for StandardHDPath)
    /// following by 4-byte BE values
    fn to_bytes(&self) -> Vec<u8> {
        let len = self.len();
        let mut buf = Vec::with_capacity(1 + 4 * (len as usize));
        buf.push(len);
        for i in 0..len {
            buf.write_u32::<BigEndian>(self.get(i)
                .expect(format!("No valut at {}", i).as_str())
                .to_raw()).unwrap();
        }
        buf
    }

}