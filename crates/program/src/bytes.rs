/// Re-interprets `&[u8]` as `&T`. The type is already 8 bytes aligned
///
/// ## Failure
///
/// * If the slice's length isn’t exactly the size of the new type
#[inline]
pub fn try_from_bytes<T: Copy>(s: &[u8]) -> Option<&T> {
    if s.len() != std::mem::size_of::<T>() {
        None
    } else {
        Some(unsafe { &*(s.as_ptr() as *const T) })
    }
}

/// Re-interprets `&mut [u8]` as `&mut T`.
///
/// ## Failure
///
/// * If the slice isn't aligned for the new type
/// * If the slice's length isn’t exactly the size of the new type
#[inline]
pub fn try_from_bytes_mut<T: Copy>(s: &mut [u8]) -> Option<&mut T> {
    if s.len() != std::mem::size_of::<T>() {
        None
    } else {
        Some(unsafe { &mut *(s.as_mut_ptr() as *mut T) })
    }
}
