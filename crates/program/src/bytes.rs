use aligned::{Aligned, A8};

/// Re-interprets `&[u8]` as `&T`. The type is already 8 bytes aligned
///
/// ## Failure
///
/// * If the slice's length isn’t exactly the size of the new type
#[inline]
pub fn try_from_bytes<T: Copy>(s: &[u8]) -> Option<&T> {
    let aligned = Aligned::<A8, &[u8]>(s);
    if aligned.len() != std::mem::size_of::<Aligned<A8, T>>() {
        None
    } else {
        Some(unsafe { &*(aligned.as_ptr() as *const Aligned<A8, T>) })
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
    let aligned = Aligned::<A8, &mut [u8]>(s);
    if aligned.len() != std::mem::size_of::<Aligned<A8, T>>() {
        None
    } else {
        Some(unsafe { &mut *(s.as_mut_ptr() as *mut Aligned<A8, T>) })
    }
}
