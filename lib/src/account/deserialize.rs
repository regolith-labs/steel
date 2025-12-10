use bytemuck::Pod;
use pinocchio::program_error::ProgramError;
pub trait Discriminator {
    fn discriminator() -> u8;
}

pub trait AccountDeserialize {
    fn try_from_bytes(data: &[u8]) -> Result<&Self, ProgramError>;
    fn try_from_bytes_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError>;
}

impl<T> AccountDeserialize for T
where
    T: Discriminator + Pod,
{
    fn try_from_bytes(data: &[u8]) -> Result<&Self, ProgramError> {
        if Self::discriminator().ne(&data[0]) {
            return Err(pinocchio::program_error::ProgramError::InvalidAccountData);
        }
        bytemuck::try_from_bytes::<Self>(&data[8..]).or(Err(
            pinocchio::program_error::ProgramError::InvalidAccountData,
        ))
    }

    fn try_from_bytes_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if Self::discriminator().ne(&data[0]) {
            return Err(pinocchio::program_error::ProgramError::InvalidAccountData);
        }
        bytemuck::try_from_bytes_mut::<Self>(&mut data[8..]).or(Err(
            pinocchio::program_error::ProgramError::InvalidAccountData,
        ))
    }
}

/// Account data is sometimes stored via a header and body type,
/// where the former resolves the type of the latter (e.g. merkle trees with a generic size const).
/// This trait parses a header type from the first N bytes of some data, and returns the remaining
/// bytes, which are then available for further processing.
///
/// See module-level tests for example usage.
pub trait AccountHeaderDeserialize {
    fn try_header_from_bytes(data: &[u8]) -> Result<(&Self, &[u8]), ProgramError>;
    fn try_header_from_bytes_mut(data: &mut [u8]) -> Result<(&mut Self, &mut [u8]), ProgramError>;
}

impl<T> AccountHeaderDeserialize for T
where
    T: Discriminator + Pod,
{
    fn try_header_from_bytes(data: &[u8]) -> Result<(&Self, &[u8]), ProgramError> {
        if Self::discriminator().ne(&data[0]) {
            return Err(pinocchio::program_error::ProgramError::InvalidAccountData);
        }
        let (prefix, remainder) = data[8..].split_at(std::mem::size_of::<T>());
        Ok((
            bytemuck::try_from_bytes::<Self>(prefix).or(Err(
                pinocchio::program_error::ProgramError::InvalidAccountData,
            ))?,
            remainder,
        ))
    }

    fn try_header_from_bytes_mut(data: &mut [u8]) -> Result<(&mut Self, &mut [u8]), ProgramError> {
        let (prefix, remainder) = data[8..].split_at_mut(std::mem::size_of::<T>());
        Ok((
            bytemuck::try_from_bytes_mut::<Self>(prefix).or(Err(
                pinocchio::program_error::ProgramError::InvalidAccountData,
            ))?,
            remainder,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::AccountDeserialize;

    use super::*;
    use bytemuck::{Pod, Zeroable};

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct GenericallySizedType<const N: usize> {
        field: [u32; N],
    }

    unsafe impl<const N: usize> Zeroable for GenericallySizedType<N> {}
    unsafe impl<const N: usize> Pod for GenericallySizedType<N> {}

    #[repr(C)]
    #[derive(Copy, Clone, Zeroable, Pod)]
    struct GenericallySizedTypeHeader {
        field_len: u64,
    }

    impl Discriminator for GenericallySizedTypeHeader {
        fn discriminator() -> u8 {
            0
        }
    }

    #[test]
    fn account_headers() {
        let mut data = [0u8; 32];
        data[8] = 4;
        data[16] = 5;
        let (_foo_header, foo) = GenericallySizedTypeHeader::try_header_from_bytes(&data)
            .map(|(header, remainder)| {
                let foo = match header.field_len {
                    4 => bytemuck::try_from_bytes::<GenericallySizedType<4>>(remainder).unwrap(),
                    x => panic!("{}", format!("unknown field len, {x}")),
                };
                (header, foo)
            })
            .unwrap();
        assert_eq!(5, foo.field[0]);
    }

    #[repr(C)]
    #[derive(Copy, Clone, Zeroable, Pod)]
    struct TestType {
        field0: u64,
        field1: u64,
    }

    impl Discriminator for TestType {
        fn discriminator() -> u8 {
            7
        }
    }

    #[test]
    fn account_deserialize() {
        let mut data = [0u8; 24];
        data[0] = 7;
        data[8] = 42;
        data[16] = 43;
        let foo = TestType::try_from_bytes(&data).unwrap();
        assert_eq!(42, foo.field0);
        assert_eq!(43, foo.field1);
    }
}
