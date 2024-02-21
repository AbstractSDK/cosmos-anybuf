use anybuf::{Anybuf, Bufany};

// DenomUnit represents a struct that describes a given
// denomination unit of the basic token.
#[derive(Clone, Debug)]
pub struct DenomUnit {
    // denom represents the string name of the given denom unit (e.g uatom).
    pub denom: String, // 1
    // exponent represents power of 10 exponent that one must
    // raise the base_denom to in order to equal the given DenomUnit's denom
    // 1 denom = 10^exponent base_denom
    // (e.g. with a base_denom of uatom, one can create a DenomUnit of 'atom' with
    // exponent = 6, thus: 1 atom = 10^6 uatom).
    pub exponent: u32, // u32
    // aliases is a list of string aliases for the given denom
    pub aliases: Vec<String>, // 3
}

impl DenomUnit {
    pub fn new(denom: String, exponent: u32, aliases: Vec<String>) -> Self {
        Self {
            denom,
            exponent,
            aliases,
        }
    }

    pub fn to_anybuf(&self) -> Anybuf {
        Anybuf::new()
            .append_string(1, &self.denom)
            .append_uint32(2, self.exponent)
            .append_repeated_string(3, &self.aliases)
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let bufany = Bufany::deserialize(&buf).ok()?;
        Some(Self {
            denom: bufany.string(1)?,
            exponent: bufany.uint32(2)?,
            aliases: bufany.repeated_string(3).ok()?,
        })
    }

    pub fn from_repeated_bytes(repeated_bytes: Vec<Vec<u8>>) -> Option<Vec<Self>> {
        repeated_bytes.into_iter().map(Self::from_buf).collect()
    }
}

impl From<cosmwasm_std::DenomUnit> for DenomUnit {
    fn from(value: cosmwasm_std::DenomUnit) -> Self {
        Self {
            denom: value.denom,
            exponent: value.exponent,
            aliases: value.aliases,
        }
    }
}

// Metadata represents a struct that describes
// a basic token.
#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct Metadata {
    pub description: String, // 1
    // denom_units represents the list of DenomUnit's for a given coin
    pub denom_units: Vec<DenomUnit>, // 2

    // base represents the base denom (should be the DenomUnit with exponent = 0).
    pub base: String, // 3

    // display indicates the suggested denom that should be
    // displayed in clients.
    pub display: String, // 4

    // name defines the name of the token (eg: Cosmos Atom)
    //
    // Since: cosmos-sdk 0.43
    pub name: String, // 5

    // symbol is the token symbol usually shown on exchanges (eg: ATOM). This can
    // be the same as the display.
    //
    // Since: cosmos-sdk 0.43
    pub symbol: String, // 6

    // URI to a document (on or off-chain) that contains additional information. Optional.
    //
    // Since: cosmos-sdk 0.46
    pub URI: String, // 7

    // URIHash is a sha256 hash of a document pointed by URI. It's used to verify that
    // the document didn't change. Optional.
    //
    // Since: cosmos-sdk 0.46
    pub URIHASH: String, //8
}

impl Metadata {
    pub fn new(
        description: String,
        denom_units: Vec<DenomUnit>,
        base: String,
        display: String,
        name: String,
        symbol: String,
        #[allow(non_snake_case)] URI: String,
        #[allow(non_snake_case)] URIHASH: String,
    ) -> Self {
        Self {
            description,
            denom_units,
            base,
            display,
            name,
            symbol,
            URI,
            URIHASH,
        }
    }

    pub fn to_anybuf(&self) -> Anybuf {
        let denom_units_anybuf: Vec<Anybuf> =
            self.denom_units.iter().map(DenomUnit::to_anybuf).collect();
        Anybuf::new()
            .append_string(1, &self.description)
            .append_repeated_message(2, &denom_units_anybuf)
            .append_string(3, &self.base)
            .append_string(4, &self.display)
            .append_string(5, &self.name)
            .append_string(6, &self.symbol)
            .append_string(7, &self.URI)
            .append_string(8, &self.URIHASH)
    }

    pub fn to_buf(&self) -> Vec<u8> {
        self.to_anybuf().into_vec()
    }

    pub fn from_bufany(bufany: Bufany) -> Option<Self> {
        let denom_units = DenomUnit::from_repeated_bytes(bufany.repeated_bytes(2)?)?;
        Some(Self {
            description: bufany.string(1)?,
            denom_units,
            base: bufany.string(3)?,
            display: bufany.string(4)?,
            name: bufany.string(5)?,
            symbol: bufany.string(6)?,
            URI: bufany.string(7)?,
            URIHASH: bufany.string(8)?,
        })
    }

    pub fn from_buf(buf: Vec<u8>) -> Option<Self> {
        let bufany = Bufany::deserialize(&buf).ok()?;
        Self::from_bufany(bufany)
    }
}

impl From<cosmwasm_std::DenomMetadata> for Metadata {
    fn from(value: cosmwasm_std::DenomMetadata) -> Self {
        Self {
            description: value.description,
            denom_units: value.denom_units.into_iter().map(Into::into).collect(),
            base: value.base,
            display: value.display,
            name: value.name,
            symbol: value.symbol,
            URI: value.uri,
            URIHASH: value.uri_hash,
        }
    }
}
