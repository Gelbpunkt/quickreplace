use memchr::memmem::Finder;

#[inline]
pub fn replace_exact_bytes(bytes: &mut [u8], find: &[u8], replace: &[u8]) {
    if find.len() != replace.len() {
        panic!("find and replace must be of equal length");
    }

    unsafe { replace_exact_bytes_unchecked(bytes, find, replace) }
}

pub unsafe fn replace_exact_bytes_unchecked(bytes: &mut [u8], find: &[u8], replace: &[u8]) {
    let finder = Finder::new(find);
    let mut offset = 0;

    while let Some(idx) = finder.find(unsafe { bytes.get_unchecked(offset..) }) {
        offset += idx;

        for (idx_offset, i) in replace.iter().enumerate() {
            *bytes.get_unchecked_mut(offset + idx_offset) = *i;
        }
    }
}

#[inline]
pub fn replace_exact_string(string: &mut str, find: &str, replace: &str) {
    // SAFETY: We are replacing with valid UTF-8
    replace_exact_bytes(
        unsafe { string.as_bytes_mut() },
        find.as_bytes(),
        replace.as_bytes(),
    )
}

#[inline]
pub unsafe fn replace_exact_string_unchecked(string: &mut str, find: &str, replace: &str) {
    replace_exact_bytes_unchecked(string.as_bytes_mut(), find.as_bytes(), replace.as_bytes())
}

#[test]
fn test_replace() {
    let mut a = String::from("hello world");
    unsafe { replace_exact_string_unchecked(a.as_mut_str(), "world", "steve") };
    assert_eq!(a.as_str(), "hello steve");
}
