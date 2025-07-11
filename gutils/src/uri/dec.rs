use std::borrow::Cow;
use std::panic::panic_any;
use std::string::FromUtf8Error;

#[inline]
pub fn from_hex_digit(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => Some(digit - b'0'),
        b'A'..=b'F' => Some(digit - b'A' + 10),
        b'a'..=b'f' => Some(digit - b'a' + 10),
        _ => None,
    }
}

/// Decode percent-encoded string assuming UTF-8 encoding.
///
/// If you need a `String`, call `.into_owned()` (not `.to_owned()`).
///
/// Unencoded `+` is preserved literally, and _not_ changed to a space.
#[inline]
pub fn decode(data: &str) -> Result<Cow<'_, str>, FromUtf8Error> {
    match decode_binary(data.as_bytes()) {
        Cow::Borrowed(_) => Ok(Cow::Borrowed(data)),
        Cow::Owned(s) => Ok(Cow::Owned(String::from_utf8(s)?)),
    }
}

/// Decode percent-encoded string as binary data, in any encoding.
///
/// Unencoded `+` is preserved literally, and _not_ changed to a space.
#[must_use]
pub fn decode_binary(data: &[u8]) -> Cow<'_, [u8]> {
    let offset = data.iter().take_while(|&&c| c != b'%').count();
    if offset >= data.len() {
        return Cow::Borrowed(data);
    }

    let mut decoded = Vec::new();
    if decoded.try_reserve(data.len()).is_err() {
        panic_any("OOM"); // more efficient codegen than built-in OOM handler
    }
    let mut out = NeverRealloc(&mut decoded);

    let (ascii, mut data) = data.split_at(offset);
    out.extend_from_slice(ascii);

    loop {
        let mut parts = data.splitn(2, |&c| c == b'%');
        // first the decoded non-% part
        let non_escaped_part = parts.next().unwrap();
        let rest = parts.next();
        if rest.is_none() && out.0.is_empty() {
            // if empty there were no '%' in the string
            return data.into();
        }
        out.extend_from_slice(non_escaped_part);

        // then decode one %xx
        match rest {
            Some(rest) => {
                if let Some(&[first, second]) = rest.get(0..2) {
                    if let Some(first_val) = from_hex_digit(first) {
                        if let Some(second_val) = from_hex_digit(second) {
                            out.push((first_val << 4) | second_val);
                            data = &rest[2..];
                        } else {
                            out.extend_from_slice(&[b'%', first]);
                            data = &rest[1..];
                        }
                    } else {
                        out.push(b'%');
                        data = rest;
                    }
                } else {
                    // too short
                    out.push(b'%');
                    out.extend_from_slice(rest);
                    break;
                }
            }
            None => break,
        }
    }
    Cow::Owned(decoded)
}

struct NeverRealloc<'a, T>(pub &'a mut Vec<T>);

impl<T> NeverRealloc<'_, T> {
    #[inline]
    pub fn push(&mut self, val: T) {
        // these branches only exist to remove redundant reallocation code
        // (the capacity is always sufficient)
        if self.0.len() != self.0.capacity() {
            self.0.push(val);
        }
    }

    #[inline]
    pub fn extend_from_slice(&mut self, val: &[T])
    where
        T: Clone,
    {
        if self.0.capacity() - self.0.len() >= val.len() {
            self.0.extend_from_slice(val);
        }
    }
}

#[test]
fn dec_borrows() {
    assert!(matches!(decode("hello"), Ok(Cow::Borrowed("hello"))));
    assert!(matches!(decode("hello%20"), Ok(Cow::Owned(s)) if s == "hello "));
    assert!(matches!(decode("%20hello"), Ok(Cow::Owned(s)) if s == " hello"));
}
