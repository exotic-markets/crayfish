use {
    crate::Discriminator,
    aligned::{Aligned, A8},
    bytemuck::Pod,
    typhoon_program::bytes::try_from_bytes_mut,
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
    T: Pod + Discriminator,
{
    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        let dis_len = T::DISCRIMINATOR.len();
        try_from_bytes_mut(&mut data[dis_len..std::mem::size_of::<Aligned<A8, Self>>() + dis_len])
    }
}
