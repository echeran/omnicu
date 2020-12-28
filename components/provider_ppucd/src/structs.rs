// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use icu_uniset::UnicodeSet;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

//
// data key structs - the structs used directly by users of data provider
//

macro_rules! data_key {
    (uniset, $sub_category:literal, $version:tt) => {
        data_key!(icu_provider::DataCategory::Uniset, $sub_category, $version)
    };

    // this was copied over from `provider` crate, but maybe should be refactored
    ($category:expr, $sub_category:literal, $version:tt) => {
        icu_provider::DataKey {
            category: $category,
            sub_category: tinystr::tinystr16!($sub_category),
            version: $version,
        }
    };
}

pub mod key {
    use icu_provider::DataKey;
    pub const WSPACE_V1: DataKey = data_key!(uniset, "WSpace", 1);
}

/// Gets a locale-invariant default struct given a data key in this module's category.
#[cfg(feature = "invariant")]
pub(crate) fn get_invariant(data_key: &DataKey) -> Option<DataResponse<'static>> {
    use crate::invariant::make_inv_response;
    match *data_key {
        key::WSPACE_V1 => make_inv_response::<UnicodeProperty>(),
        _ => None,
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct UnicodeProperties {
    pub props: Vec<UnicodeProperty>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct UnicodeProperty {
    pub name: String,
    pub inv_list: Vec<u32>,
}

impl UnicodeProperty {
    /// Default empty nameless property
    pub fn default() -> UnicodeProperty {
        UnicodeProperty {
            name: String::new(),
            inv_list: vec![],
        }
    }

    /// Converts a UnicodeSet into a corresponding UnicodeProperty struct
    /// for serde JSON de-/serialization.
    pub fn from_uniset(s: &UnicodeSet, name: &str) -> UnicodeProperty {
        let inv_list = s.get_inversion_list();
        UnicodeProperty {
            name: String::from(name),
            inv_list,
        }
    }
}

impl TryInto<UnicodeSet> for UnicodeProperty {
    type Error = crate::error::Error;
    fn try_into(self) -> Result<UnicodeSet, Self::Error> {
        UnicodeSet::from_inversion_list(self.inv_list)
            .map_err(crate::error::Error::UnisetConversion)
    }
}

#[test]
fn test_uniset_to_inv_list() {
    let inv_list: Vec<u32> = vec![
        9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240, 8287,
        8288, 12288, 12289,
    ];
    let inv_list_clone = (&inv_list).clone();
    let s: UnicodeSet = UnicodeSet::from_inversion_list(inv_list_clone).unwrap();
    let round_trip_inv_list = s.get_inversion_list();
    assert_eq!(round_trip_inv_list, inv_list);
}