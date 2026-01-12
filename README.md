# mzIdentML

Reader for [mzIdentMl-files](https://www.psidev.info/mzidentml).

## Usage
The crate is for those who are familiar with the format and want to extract information to further processing. Files can be loaded directly into memory or indexed first to support random or streaming access to the data.  
The parser is relative relaxed and only complains about missing required attributes or missing children during file loading. In case 1 or many children of the same type should be present but missing, the parser will just create an empty vector. This is checked during optional validation.


### Validation
Validating is optional and has initial support for different version of the file format.

## Development
Scattered arround the code are `TODO` comments which can be worked on. Please submit your work as PRs.

### Errors and validations
* Some errors should be split in separate errors to better handle them. Especially the validation errors.
* Errors which might occure during (de-)serialization bear no element tag. So it is hard to track where the error is located ion the file.
* The validation is not following references yet. To do so, the validation needs to have access to the (indexed) document.

### Format version
Vesion 1.0, 1.1 and 1.2 are not tested yet. However, the parsing might still work. That depends of if mandatoryelements expected in 1.3 are present in the given file.    

Full compatibility is planed along with a structure which make the elements reusable across the version if possible.
```
src/
  |
  |- 1.0
  |  |- elements
  |  |  |- element1.rs
  |  |  |- element2.rs
  |  |  |- ...
  |  |  |- elementN.rs
  |  |- document.rs
  |
  |- 1.1
  |  |- elements
  |  |  |- element2_updated_for_version1.1.rs
  |  |- document.rs (reuse elements from verson 1.0 when possible)
  |
  |- 1.2
  |  |- elements
  |  |  |- element1_updated_for_version1.2.rs
  |  |- document.rs (reuse elements from verson 1.0 and 1.1 when possible)
  |
  |- indexer.rs
  |- reader.rs
```


### Elements
The structs for the elements are completely hand crafted. In general, it would be a good idea to assemble feature complete mzIdentMl files for each version of the format and use [xml_schema_generator](https://github.com/Thomblin/xml_schema_generator) to generate the struct for each element automatically, clean them up and just implement the necessary traits.


### Test data
* [HUPO-PSI](https://www.psidev.info/mzidentml): scores_and_thresholds_1_3_0_draft.mzid
* [Douwe Schulte](https://github.com/douweschulte): `novor_v3.40.910_202512_results.mzid` & `novor_v3.40.910_202512_results.mzid`
