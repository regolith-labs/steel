#[macro_export]
macro_rules! impl_to_bytes {
    ($struct_name:ident) => {
        impl $struct_name {
            pub fn to_bytes(&self) -> &[u8] {
                bytemuck::bytes_of(self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_bytes {
    ($struct_name:ident) => {
        impl $struct_name {
            pub fn from_bytes(data: &[u8]) -> &Self {
                bytemuck::from_bytes::<Self>(data)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_instruction_from_bytes {
    ($struct_name:ident) => {
        impl $struct_name {
            pub fn try_from_bytes(
                data: &[u8],
            ) -> Result<&Self, solana_program::program_error::ProgramError> {
                bytemuck::try_from_bytes::<Self>(data).or(Err(
                    solana_program::program_error::ProgramError::InvalidInstructionData,
                ))
            }
        }
    };
}

#[macro_export]
macro_rules! account {
    ($discriminator_name:ident, $struct_name:ident) => {
        $crate::impl_to_bytes!($struct_name);

        impl $crate::Discriminator for $struct_name {
            fn discriminator() -> u8 {
                $discriminator_name::$struct_name.into()
            }
        }

        impl $crate::AccountValidation for $struct_name {
            fn assert<F>(
                &self,
                condition: F,
            ) -> Result<&Self, solana_program::program_error::ProgramError>
            where
                F: Fn(&Self) -> bool,
            {
                if !condition(self) {
                    return Err(solana_program::program_error::ProgramError::InvalidAccountData);
                }
                Ok(self)
            }
            fn assert_with_msg<F>(
                &self,
                condition: F,
                msg: &str,
            ) -> Result<&Self, solana_program::program_error::ProgramError>
            where
                F: Fn(&Self) -> bool,
            {
                if let Err(err) = $crate::assert_with_msg(
                    condition(self),
                    solana_program::program_error::ProgramError::InvalidAccountData,
                    msg,
                ) {
                    return Err(err.into());
                }
                Ok(self)
            }
            fn assert_mut<F>(
                &mut self,
                condition: F,
            ) -> Result<&mut Self, solana_program::program_error::ProgramError>
            where
                F: Fn(&Self) -> bool,
            {
                if !condition(self) {
                    return Err(solana_program::program_error::ProgramError::InvalidAccountData);
                }
                Ok(self)
            }
            fn assert_mut_with_msg<F>(
                &mut self,
                condition: F,
                msg: &str,
            ) -> Result<&mut Self, solana_program::program_error::ProgramError>
            where
                F: Fn(&Self) -> bool,
            {
                if let Err(err) = $crate::assert_with_msg(
                    condition(self),
                    solana_program::program_error::ProgramError::InvalidAccountData,
                    msg,
                ) {
                    return Err(err.into());
                }
                Ok(self)
            }
        }
    };
}

#[macro_export]
macro_rules! error {
    ($struct_name:ident) => {
        impl From<$struct_name> for solana_program::program_error::ProgramError {
            fn from(e: $struct_name) -> Self {
                solana_program::program_error::ProgramError::Custom(e as u32)
            }
        }
    };
}

#[macro_export]
macro_rules! event {
    ($struct_name:ident) => {
        $crate::impl_to_bytes!($struct_name);
        $crate::impl_from_bytes!($struct_name);
    };
}

#[macro_export]
macro_rules! instruction {
    ($discriminator_name:ident, $struct_name:ident) => {
        $crate::impl_instruction_from_bytes!($struct_name);

        impl $crate::Discriminator for $struct_name {
            fn discriminator() -> u8 {
                $discriminator_name::$struct_name as u8
            }
        }

        impl $struct_name {
            pub fn to_bytes(&self) -> Vec<u8> {
                [
                    [$discriminator_name::$struct_name as u8].to_vec(),
                    bytemuck::bytes_of(self).to_vec(),
                ]
                .concat()
            }
        }
    };
}
