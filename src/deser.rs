//! This module contains the definitions of the various enums used to
//! (de)serialize the AArch64 instruction metadata.

#![allow(clippy::upper_case_acronyms)]

use crate::InsnBitField;
use crate::InsnClass;
use crate::InsnFeatureSet;
use crate::InsnFlags;
use crate::InsnOperandClass;
use crate::InsnOperandKind;
use crate::InsnOperandQualifier;
use serde::de::value::StrDeserializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

fn deser_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let without_prefix = s.trim_start_matches("0x");
    u32::from_str_radix(without_prefix, 16).map_err(serde::de::Error::custom)
}

/// The AArch64 instruction bit field specification.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BitfieldSpec {
    pub bitfield: InsnBitField,
    pub lsb: u8,
    pub width: u8,
}

fn deser_bitfield_spec_str<'de, D>(s: &str) -> Result<BitfieldSpec, D::Error>
where
    D: Deserializer<'de>,
{
    let mut parts = s.split(':');
    let operand_kind = parts
        .next()
        .ok_or_else(|| serde::de::Error::custom("missing operand kind"))?;
    let deser = StrDeserializer::new(operand_kind);
    let bitfield = InsnBitField::deserialize(deser)?;

    let lsb = parts
        .next()
        .ok_or_else(|| serde::de::Error::custom("missing lsb"))?
        .parse::<u8>()
        .map_err(|_| serde::de::Error::custom("invalid lsb"))?;
    let width = parts
        .next()
        .ok_or_else(|| serde::de::Error::custom("missing lsb"))?
        .parse::<u8>()
        .map_err(|_| serde::de::Error::custom("invalid lsb"))?;

    Ok(BitfieldSpec {
        bitfield,
        lsb,
        width,
    })
}

fn deser_bitfield_spec_vec<'de, D>(deserializer: D) -> Result<Vec<BitfieldSpec>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vec<String> = Vec::deserialize(deserializer)?;
    v.into_iter()
        .map(|s| deser_bitfield_spec_str::<D>(s.as_str()))
        .collect()
}

/// The AArch64 instruction operand.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InsnOperand {
    pub kind: InsnOperandKind,
    pub class: InsnOperandClass,
    pub qualifiers: Vec<InsnOperandQualifier>,
    #[serde(deserialize_with = "deser_bitfield_spec_vec")]
    pub bit_fields: Vec<BitfieldSpec>,
}

/// The AArch64 instruction definition.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Insn {
    pub mnemonic: String,
    #[serde(deserialize_with = "deser_hex")]
    pub opcode: u32,
    #[serde(deserialize_with = "deser_hex")]
    pub mask: u32,
    pub class: InsnClass,
    pub feature_set: InsnFeatureSet,
    pub operands: Vec<InsnOperand>,
    pub flags: InsnFlags,
    pub index: u32,
}
