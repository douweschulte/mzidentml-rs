use std::io::BufRead;

use crate::{elements::mz_ident_ml::MzIdentMl, error::Error};

/// Controlled vocabularies needed for mzIdentML validation.
pub mod controlled_vocabularies;
pub mod elements;
pub mod error;
// pub mod index;
// pub mod indexer;
// pub mod reader;
/// Parsers for several attributes in the elements
pub mod parsing;
pub mod utils;

pub fn read<R: BufRead>(reader: R) -> Result<MzIdentMl, Error> {
    let mut mzid_deserializer = quick_xml::de::Deserializer::from_reader(reader);
    serde_path_to_error::deserialize::<_, MzIdentMl>(&mut mzid_deserializer)
        .map_err(Error::Deserialization)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::read;

    pub static MZID_FILE_PATHS: [&str; 3] = [
        "./test_data/scores_and_thresholds_1_3_0_draft.mzid",
        "./test_data/novor_v3.40.910_202512_results.mzid",
        "./test_data/byonic_v5.1.mzid",
    ];

    #[test]
    fn test_read() {
        for path in MZID_FILE_PATHS {
            let reader = BufReader::new(File::open(path).unwrap());

            let mzid_res = read(reader);
            assert!(mzid_res.is_ok(), "{path}: {}", mzid_res.unwrap_err())
        }
    }
}
