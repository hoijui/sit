//! Binary identifier encodings

use serde_derive::{Deserialize, Serialize};
use data_encoding_macro::*;

/// Available encodings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Encoding {
    /// [Base32] encoding
    ///
    /// [Base32]: https://en.wikipedia.org/wiki/Base32
    #[serde(rename = "base32")]
    Base32,
}


impl Default for Encoding {
    fn default() -> Self {
        Encoding::Base32
    }
}

use data_encoding;

use std::ops::Deref;

impl Deref for Encoding {
    type Target = data_encoding::Encoding;

    fn deref(&self) -> &Self::Target {
        match self {
            &Encoding::Base32 => &BASE32_DASHPAD,
        }
    }
}

const BASE32_DASHPAD: data_encoding::Encoding = new_encoding!{
    symbols: "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
    padding: '-',
};

