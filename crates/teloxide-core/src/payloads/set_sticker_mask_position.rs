//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{MaskPosition, True};

impl_payload! {
    /// Use this method to change the mask position of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns True on success.
    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub SetStickerMaskPosition (SetStickerMaskPositionSetters) => True {
        required {
            /// File identifier of the sticker
            pub sticker: String [into],
        }
        optional {
            /// A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
            pub mask_position: MaskPosition,
        }
    }
}