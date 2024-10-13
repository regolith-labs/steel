use bytemuck::Pod;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

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
            return Err(solana_program::program_error::ProgramError::InvalidAccountData);
        }
        bytemuck::try_from_bytes(&data[8..]).or(Err(
            solana_program::program_error::ProgramError::InvalidAccountData,
        ))
    }

    fn try_from_bytes_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if Self::discriminator().ne(&data[0]) {
            return Err(solana_program::program_error::ProgramError::InvalidAccountData);
        }
        bytemuck::try_from_bytes_mut(&mut data[8..]).or(Err(
            solana_program::program_error::ProgramError::InvalidAccountData,
        ))
    }
}

/// Interface for using header data to parse the remainder of an account's bytes.
pub trait FromHeader<'a, H: AccountDeserialize + 'a>: Sized {
    /// Defines how to get a [Self] from a header H and any remaining bytes.
    /// You define this, but you do not call it directly.
    /// Use [FromHeader::from_account_data].
    fn try_from_header(header: &'a H, data: &'a [u8]) -> Result<Self, ProgramError>;

    /// Convert raw data to the underlying data type
    fn from_account_data(data: &'a [u8]) -> Result<Self, ProgramError> {
        let (header_bytes, remainder) = data.split_at(8 + std::mem::size_of::<H>());
        H::try_from_bytes(header_bytes).and_then(|t| Self::try_from_header(t, remainder))
    }
}

/// Mutable version of [FromHeader].
pub trait FromHeaderMut<'a, H: AccountDeserialize + 'a>: Sized {
    /// Defines how to get a [Self] from mutable acces to a header H and any remaining bytes.
    /// You define this, but you do not call it directly.
    /// Use [FromHeaderMut::from_account_data_mut].
    fn try_from_header_mut(header: &'a mut H, data: &'a mut [u8]) -> Result<Self, ProgramError>;

    fn from_account_data_mut(data: &'a mut [u8]) -> Result<Self, ProgramError> {
        let (header_bytes, remainder) = data.split_at_mut(8 + std::mem::size_of::<H>());
        H::try_from_bytes_mut(header_bytes).and_then(|t| Self::try_from_header_mut(t, remainder))
    }
}

pub trait AccountValidation {
    fn assert<F>(&self, condition: F) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool;

    fn assert_mut<F>(&mut self, condition: F) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool;

    fn assert_with_msg<F>(&self, condition: F, msg: &str) -> Result<&Self, ProgramError>
    where
        F: Fn(&Self) -> bool;

    fn assert_mut_with_msg<F>(
        &mut self,
        condition: F,
        msg: &str,
    ) -> Result<&mut Self, ProgramError>
    where
        F: Fn(&Self) -> bool;
}

pub trait AccountInfoValidation {
    fn is_signer(&self) -> Result<&Self, ProgramError>;
    fn is_writable(&self) -> Result<&Self, ProgramError>;
    fn is_executable(&self) -> Result<&Self, ProgramError>;
    fn is_empty(&self) -> Result<&Self, ProgramError>;
    fn is_type<T: Discriminator>(&self, program_id: &Pubkey) -> Result<&Self, ProgramError>;
    fn is_program(&self, program_id: &Pubkey) -> Result<&Self, ProgramError>;
    fn is_sysvar(&self, sysvar_id: &Pubkey) -> Result<&Self, ProgramError>;
    fn has_address(&self, address: &Pubkey) -> Result<&Self, ProgramError>;
    fn has_owner(&self, program_id: &Pubkey) -> Result<&Self, ProgramError>;
    fn has_seeds(&self, seeds: &[&[u8]], program_id: &Pubkey) -> Result<&Self, ProgramError>;
}

pub trait Discriminator {
    fn discriminator() -> u8;
}

/// Performs:
/// 1. Program owner check
/// 2. Discriminator byte check
/// 3. Checked bytemuck conversion of account data to &T or &mut T.
pub trait AsAccount {
    fn as_account<T>(&self, program_id: &Pubkey) -> Result<&T, ProgramError>
    where
        T: AccountDeserialize + Discriminator + Pod;

    fn as_account_mut<T>(&self, program_id: &Pubkey) -> Result<&mut T, ProgramError>
    where
        T: AccountDeserialize + Discriminator + Pod;
}

#[cfg(feature = "spl")]
pub trait AsSplToken {
    fn as_mint(&self) -> Result<spl_token::state::Mint, ProgramError>;
    fn as_token_account(&self) -> Result<spl_token::state::Account, ProgramError>;
    fn as_associated_token_account(
        &self,
        owner: &Pubkey,
        mint: &Pubkey,
    ) -> Result<spl_token::state::Account, ProgramError>;
}

pub trait ProgramOwner {
    fn owner() -> Pubkey;
}

#[cfg(test)]
mod tests {
    use crate::{
        try_cast_slice, try_cast_slice_mut, try_cast_slice_mut_with_remainder,
        try_cast_slice_with_remainder,
    };

    use super::*;
    use bytemuck::{Pod, Zeroable};

    /// Some [Pod] data that stores the lengths of two fixed-size arrays whose lengths
    /// could differ across different accounts.
    #[repr(C)]
    #[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
    struct SliceHeader {
        some_metadata: [u8; 32],
        num_players: u64,
        num_mints: u64,
    }

    impl SliceHeader {
        pub fn total_required_size(&self) -> usize {
            8 + std::mem::size_of::<Self>()
                + std::mem::size_of::<Pubkey>() * self.num_mints as usize
                + std::mem::size_of::<Pubkey>() * self.num_players as usize
        }
    }

    impl Discriminator for SliceHeader {
        fn discriminator() -> u8 {
            0
        }
    }

    /// Some [Pod] data paired with two [Pubkey] collections, accessed as slices.
    /// The header stores the expected length of both collections.
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct SliceAccount<'a> {
        header: &'a SliceHeader,
        players: &'a [Pubkey],
        mints: &'a [Pubkey],
    }

    // A mutable version of [SliceAccount].
    #[repr(C)]
    struct SliceAccountMut<'a> {
        header: &'a mut SliceHeader,
        players: &'a mut [Pubkey],
        mints: &'a mut [Pubkey],
    }

    // This feels very "macro"-able
    impl<'a> FromHeader<'a, SliceHeader> for SliceAccount<'a> {
        fn try_from_header(header: &'a SliceHeader, data: &'a [u8]) -> Result<Self, ProgramError> {
            let (players, data) = try_cast_slice_with_remainder(data, header.num_players as usize)?;
            let mints = try_cast_slice(data, header.num_mints as usize)?;
            Ok(Self {
                header,
                players,
                mints,
            })
        }
    }

    impl<'a> FromHeaderMut<'a, SliceHeader> for SliceAccountMut<'a> {
        fn try_from_header_mut(
            header: &'a mut SliceHeader,
            data: &'a mut [u8],
        ) -> Result<Self, ProgramError> {
            let (players, data) =
                try_cast_slice_mut_with_remainder(data, header.num_players as usize)?;
            let mints = try_cast_slice_mut(data, header.num_mints as usize)?;
            Ok(Self {
                header,
                players,
                mints,
            })
        }
    }

    fn generate_slice_account(num_players: u64, num_mints: u64) -> Vec<u8> {
        let header = SliceHeader {
            some_metadata: [1u8; 32],
            num_players,
            num_mints,
        };
        let header_bytes = bytemuck::bytes_of(&header).to_vec();
        let mut data = vec![0u8; header.total_required_size()];
        data[8..header_bytes.len() + 8].copy_from_slice(&header_bytes);
        data
    }

    #[test]
    fn account_headers() {
        let mut data = generate_slice_account(3, 2);
        // Deserialize works?
        let foo = SliceAccount::from_account_data(&data).unwrap();
        assert_eq!(3, foo.header.num_players);
        assert_eq!(2, foo.header.num_mints);
        assert_eq!(Pubkey::default(), foo.players[0]);
        assert_eq!(Pubkey::default(), foo.mints[0]);

        // Mutation works?
        let foo = SliceAccountMut::from_account_data_mut(&mut data).unwrap();
        let new_player = Pubkey::new_unique();
        foo.players[0] = new_player;
        let new_mint = Pubkey::new_unique();
        foo.mints[0] = new_mint;
        let foo = SliceAccount::from_account_data(&data).unwrap();
        assert_eq!(new_player, foo.players[0]);
        assert_eq!(new_mint, foo.mints[0]);
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Zeroable, Pod)]
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

        // Cast a slice of `TestType` (no discriminator)
        let data_len = 5usize;
        let mut data = vec![0u8; std::mem::size_of::<TestType>() * data_len];
        let _: &[TestType] = try_cast_slice::<TestType>(&data, data_len).unwrap();
        let foo: &mut [TestType] = try_cast_slice_mut::<TestType>(&mut data, data_len).unwrap();
        foo[4].field0 = 123;
        let foo: &[TestType] = try_cast_slice::<TestType>(&data, data_len).unwrap();
        assert_eq!(foo[4].field0, 123);
    }

    #[derive(Debug, Clone)]
    struct SliceAccountOwned {
        header: SliceHeader,
        players: Vec<Pubkey>,
        mints: Vec<Pubkey>,
    }

    impl<'a> FromHeader<'a, SliceHeader> for SliceAccountOwned {
        fn try_from_header(header: &'a SliceHeader, data: &'a [u8]) -> Result<Self, ProgramError> {
            let borrowed: SliceAccount<'a> = SliceAccount::try_from_header(header, data)?;
            Ok(Self {
                header: *borrowed.header,
                players: borrowed.players.to_vec(),
                mints: borrowed.mints.to_vec(),
            })
        }
    }

    #[test]
    fn converting_to_client_types() {
        let mut data = generate_slice_account(3, 4);
        // Deserialize works?
        let foo = SliceAccountOwned::from_account_data(&data).unwrap();
        assert_eq!(3, foo.header.num_players);
        assert_eq!(4, foo.header.num_mints);
        assert_eq!(Pubkey::default(), foo.players[2]);
        assert_eq!(Pubkey::default(), foo.mints[2]);

        // Mutation works?
        let foo = SliceAccountMut::from_account_data_mut(&mut data).unwrap();
        let new_player = Pubkey::new_unique();
        foo.players[2] = new_player;
        let new_mint = Pubkey::new_unique();
        foo.mints[2] = new_mint;
        let foo = SliceAccountOwned::from_account_data(&data).unwrap();
        assert_eq!(new_player, foo.players[2]);
        assert_eq!(new_mint, foo.mints[2]);
    }
}
