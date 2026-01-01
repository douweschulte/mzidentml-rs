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

    #[test]
    fn test_read() {
        let reader = BufReader::new(
            File::open("./test_data/scores_and_thresholds_1_3_0_draft.mzid").unwrap(),
        );

        let mzid_res = read(reader);
        assert!(mzid_res.is_ok(), "{}", mzid_res.unwrap_err())
    }
}
