#![cfg(test)]

use crate::deser::Insn;

#[test]
fn test_json() {
    const JSON: &str = r#"
    [
        {
          "mnemonic": "adc",
          "aliases": [],
          "opcode": "0x1a000000",
          "mask": "0x7fe0fc00",
          "class": "ADDSUB_CARRY",
          "feature_set": "V8",
          "description": "Add/subtract (with carry)",
          "operands": [
            {
              "kind": "Rd",
              "class": "INT_REG",
              "qualifiers": [
                "W",
                "X"
              ],
              "bit_fields": [
                "Rd:0:5"
              ]
            },
            {
              "kind": "Rn",
              "class": "INT_REG",
              "qualifiers": [
                "W",
                "X"
              ],
              "bit_fields": [
                "Rn:5:5"
              ]
            },
            {
              "kind": "Rm",
              "class": "INT_REG",
              "qualifiers": [
                "W",
                "X"
              ],
              "bit_fields": [
                "Rm:16:5"
              ]
            }
          ],
          "flags": "HAS_SF_FIELD",
          "index": 0
        },
        {
          "mnemonic": "adcs",
          "aliases": [],
          "opcode": "0x3a000000",
          "mask": "0x7fe0fc00",
          "class": "ADDSUB_CARRY",
          "feature_set": "V8",
          "description": "Add/subtract (with carry)",
          "operands": [
            {
              "kind": "Rd",
              "class": "INT_REG",
              "qualifiers": [
                "W",
                "X"
              ],
              "bit_fields": [
                "Rd:0:5"
              ]
            },
            {
              "kind": "Rn",
              "class": "INT_REG",
              "qualifiers": [
                "W",
                "X"
              ],
              "bit_fields": [
                "Rn:5:5"
              ]
            },
            {
              "kind": "Rm",
              "class": "INT_REG",
              "qualifiers": [
                "W",
                "X"
              ],
              "bit_fields": [
                "Rm:16:5"
              ]
            }
          ],
          "flags": "HAS_SF_FIELD",
          "index": 1
        }
    ]
    "#;

    let instructions: Vec<Insn> = serde_json::from_str(JSON).unwrap();
    assert_eq!(instructions.len(), 2);
}
