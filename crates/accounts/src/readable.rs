use {crate::Owner, bytemuck::Pod};

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
        bytemuck::try_from_bytes(&data[..std::mem::size_of::<Self>()]).ok() //TODO replace when we have alignement trait
    }

    //TODO replace when we have alignement trait
    fn read_mut(data: &mut [u8]) -> Option<&mut Self> {
        bytemuck::try_from_bytes_mut(&mut data[..std::mem::size_of::<Self>()]).ok()
    }
}
