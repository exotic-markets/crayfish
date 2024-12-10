use {
    aligned::{Aligned, A8},
    bytemuck::Pod,
    crayfish_program::bytes::try_from_bytes_mut,
};

pub trait ReadMut {
    fn read_mut(data: &mut [u8]) -> Option<&mut Self>;
}

impl ReadMut for [u8] {
    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        Some(data)
    }
}

impl<T> ReadMut for T
where
    T: Pod,
{
    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        try_from_bytes_mut(&mut data[..std::mem::size_of::<Aligned<A8, Self>>()])
    }
}
