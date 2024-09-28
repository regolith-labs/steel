use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum {name_typecase}Instruction {
    Initialize = 0,
    Add = 1
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Add {
    pub amount: [u8; 8]
}

instruction!({name_typecase}Instruction, Initialize);
instruction!({name_typecase}Instruction, Add);
