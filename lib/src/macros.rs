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
            ) -> Result<&Self, pinocchio::program_error::ProgramError> {
                bytemuck::try_from_bytes::<Self>(data).or(Err(
                    pinocchio::program_error::ProgramError::InvalidInstructionData,
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
            #[track_caller]
            fn assert<F>(
                &self,
                condition: F,
            ) -> Result<&Self, pinocchio::program_error::ProgramError>
            where
                F: Fn(&Self) -> bool,
            {
                if !condition(self) {
                    return Err(trace(
                        "Account data is invalid",
                        pinocchio::program_error::ProgramError::InvalidAccountData,
                    ));
                }
                Ok(self)
            }

            #[track_caller]
            fn assert_err<F>(
                &self,
                condition: F,
                err: pinocchio::program_error::ProgramError,
            ) -> Result<&Self, pinocchio::program_error::ProgramError>
            where
                F: Fn(&Self) -> bool,
            {
                if !condition(self) {
                    return Err(trace("Account data is invalid", err));
                }
                Ok(self)
            }

            #[track_caller]
            fn assert_msg<F>(
                &self,
                condition: F,
                msg: &str,
            ) -> Result<&Self, pinocchio::program_error::ProgramError>
            where
                F: Fn(&Self) -> bool,
            {
                if !condition(self) {
                    return Err(trace(
                        format!("Account data is invalid: {}", msg).as_str(),
                        pinocchio::program_error::ProgramError::InvalidAccountData,
                    ));
                }
                Ok(self)
            }

            #[track_caller]
            fn assert_mut<F>(
                &mut self,
                condition: F,
            ) -> Result<&mut Self, pinocchio::program_error::ProgramError>
            where
                F: Fn(&Self) -> bool,
            {
                if !condition(self) {
                    return Err(trace(
                        "Account data is invalid",
                        pinocchio::program_error::ProgramError::InvalidAccountData,
                    ));
                }
                Ok(self)
            }

            #[track_caller]
            fn assert_mut_err<F>(
                &mut self,
                condition: F,
                err: pinocchio::program_error::ProgramError,
            ) -> Result<&mut Self, pinocchio::program_error::ProgramError>
            where
                F: Fn(&Self) -> bool,
            {
                if !condition(self) {
                    return Err(trace("Account data is invalid", err));
                }
                Ok(self)
            }

            #[track_caller]
            fn assert_mut_msg<F>(
                &mut self,
                condition: F,
                msg: &str,
            ) -> Result<&mut Self, pinocchio::program_error::ProgramError>
            where
                F: Fn(&Self) -> bool,
            {
                if !condition(self) {
                    return Err(trace(
                        format!("Account data is invalid: {}", msg).as_str(),
                        pinocchio::program_error::ProgramError::InvalidAccountData,
                    ));
                }
                Ok(self)
            }
        }
    };
}

#[macro_export]
macro_rules! error {
    ($struct_name:ident) => {
        impl From<$struct_name> for pinocchio::program_error::ProgramError {
            fn from(e: $struct_name) -> Self {
                pinocchio::program_error::ProgramError::Custom(e as u32)
            }
        }
    };
}

#[macro_export]
macro_rules! event {
    ($struct_name:ident) => {
        $crate::impl_to_bytes!($struct_name);
        $crate::impl_from_bytes!($struct_name);

        impl $crate::Loggable for $struct_name {
            fn log(&self) {
                pinocchio::log::sol_log_data(&[self.to_bytes()]);
            }

            fn log_return(&self) {
                pinocchio::program::set_return_data(self.to_bytes());
            }
        }
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
