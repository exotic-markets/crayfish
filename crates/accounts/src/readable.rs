use {
    crate::Owner,
    aligned::{Aligned, A8},
    bytemuck::Pod,
    crayfish_program::bytes::{try_from_bytes, try_from_bytes_mut},
};

pub trait Readable {
    fn read(data: &[u8]) -> Option<&Self>;
    fn read_mut(data: &mut [u8]) -> Option<&mut Self>;
}

impl Readable for [u8] {
    fn read(data: &[u8]) -> Option<&Self> {
        Some(data)
    }

    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        Some(data)
    }
}

impl<T> Readable for T
where
    T: Owner + Pod,
{
    fn read(data: &[u8]) -> Option<&Self> {
        try_from_bytes(&data[..std::mem::size_of::<Aligned<A8, Self>>()])
    }

    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        try_from_bytes_mut(&mut data[..std::mem::size_of::<Aligned<A8, Self>>()])
    }
}
