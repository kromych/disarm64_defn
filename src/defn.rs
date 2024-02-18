//! This module contains the definitions of the various enums,
//! using types that can be statically defined without allocations

#![allow(clippy::collapsible_else_if)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(non_snake_case, non_camel_case_types)]

use crate::BitfieldSpec;
use crate::InsnClass;
use crate::InsnFeatureSet;
use crate::InsnFlags;
use crate::InsnOperandClass;
use crate::InsnOperandKind;
use crate::InsnOperandQualifier;

/// AArch64 instruction definitions.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InsnOperand {
    pub kind: InsnOperandKind,
    pub class: InsnOperandClass,
    pub qualifiers: &'static [InsnOperandQualifier],
    pub bit_fields: &'static [BitfieldSpec],
}

/// AArch64 instruction definitions.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Insn {
    pub mnemonic: &'static str,
    pub aliases: &'static [&'static Insn],
    pub opcode: u32,
    pub mask: u32,
    pub class: InsnClass,
    pub feature_set: InsnFeatureSet,
    pub operands: &'static [InsnOperand],
    pub flags: InsnFlags,
}

/// AArch64 instruction definitions.
pub trait InsnOpcode {
    fn definition(&self) -> &'static Insn;
    fn bits(&self) -> u32;
}
