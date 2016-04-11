use std::collections::BTreeMap;
use rustc_serialize::{Decodable, json};

use types::*;
use parameters::*;
use api_call::ApiCall;

use {Result, Error};

/// Database metadata query.
///
/// [Quandl API Reference](https://www.quandl.com/docs/api#database-metadata)
///
#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseMetadataQuery {
    database_code: String,
    request_arguments: ApiArguments,
}

/// Dataset metadata query.
///
/// [Quandl API Reference](https://www.quandl.com/docs/api#dataset-metadata)
///
#[derive(Debug, Clone, PartialEq)]
pub struct DatasetMetadataQuery {
    database_code: String,
    dataset_code: String,
    request_arguments: ApiArguments,
}

/// Query to search into a database metadata list.
///
/// [Quandl API Reference](https://www.quandl.com/docs/api#database-list)
///
#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseSearch {
    request_arguments: ApiArguments,
    search_arguments: SearchArguments,
}

/// Query to search into a dataset metadata list.
///
/// [Quandl API Reference](https://www.quandl.com/docs/api#dataset-search)
///
#[derive(Debug, Clone, PartialEq)]
pub struct DatasetSearch {
    database_code: String,
    request_arguments: ApiArguments,
    search_arguments: SearchArguments,
}

/// Query a list of dataset codes from a specific database.
///
/// [Quandl API Reference](https://www.quandl.com/docs/api#dataset-list)
///
#[derive(Debug, Clone, PartialEq)]
pub struct CodeListQuery {
    database_code: String,
    request_arguments: ApiArguments,
}

/// Query the data from a specific dataset.
///
/// [Quandl API Reference](https://www.quandl.com/docs/api#data)
///
#[derive(Debug, Clone, PartialEq)]
pub struct DataQuery {
    database_code: String,
    dataset_code: String,
    data_arguments: DataArguments,
    request_arguments: ApiArguments,
}

/// Query the data and metadata from a specific dataset.
///
/// [Quandl API Reference](https://www.quandl.com/docs/api#data-and-metadata)
///
#[derive(Debug, Clone, PartialEq)]
pub struct DataAndMetadataQuery {
    database_code: String,
    dataset_code: String,
    data_arguments: DataArguments,
    request_arguments: ApiArguments,
}

impl DatabaseMetadataQuery {
    /// Create a new database metadata query.
    ///
    pub fn new<S: AsRef<str>>(database_code: S) -> Self {
        DatabaseMetadataQuery {
            database_code: database_code.as_ref().to_string(),
            request_arguments: ApiArguments::default(),
        }
    }
}

impl DatasetMetadataQuery {
    /// Create a new dataset metadata query.
    ///
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(database_code: S1, dataset_code: S2) -> Self {
        DatasetMetadataQuery {
            database_code: database_code.as_ref().to_string(),
            dataset_code: dataset_code.as_ref().to_string(),
            request_arguments: ApiArguments::default(),
        }
    }
}

impl DatabaseSearch {
    /// Create a new database search query.
    ///
    pub fn new() -> Self {
        DatabaseSearch {
            request_arguments: ApiArguments::default(),
            search_arguments: SearchArguments::default(),
        }
    }
}

impl DatasetSearch {
    /// Create a new dataset search query.
    ///
    pub fn new<S: AsRef<str>>(database_code: S) -> Self {
        DatasetSearch {
            database_code: database_code.as_ref().to_string(),
            request_arguments: ApiArguments::default(),
            search_arguments: SearchArguments::default(),
        }
    }
}

impl CodeListQuery {
    /// Create a new code list query.
    ///
    pub fn new<S: AsRef<str>>(database_code: S) -> Self {
        CodeListQuery {
            database_code: database_code.as_ref().to_string(),
            request_arguments: ApiArguments::default(),
        }
    }
}

impl DataQuery {
    /// Create a new data query.
    ///
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(database_code: S1, dataset_code: S2) -> Self {
        DataQuery {
            database_code: database_code.as_ref().to_string(),
            dataset_code: dataset_code.as_ref().to_string(),
            data_arguments: DataArguments::default(),
            request_arguments: ApiArguments::default(),
        }
    }
}

impl DataAndMetadataQuery {
    /// Create a new data and metadata query.
    ///
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(database_code: S1, dataset_code: S2) -> Self {
        DataAndMetadataQuery {
            database_code: database_code.as_ref().to_string(),
            dataset_code: dataset_code.as_ref().to_string(),
            data_arguments: DataArguments::default(),
            request_arguments: ApiArguments::default(),
        }
    }
}

impl ApiCall<DatabaseMetadata> for DatabaseMetadataQuery {
    fn send(&self) -> Result<DatabaseMetadata> {
        send_and_unwrap_json(self)
    }

    fn fmt_prefix(&self) -> Option<String> {
        Some(format!("/databases/{}.json", self.database_code))
    }

    fn fmt_arguments(&self) -> Option<String> {
        ApiParameters::fmt(self)
    }
}

impl ApiCall<DatasetMetadata> for DatasetMetadataQuery {
    fn send(&self) -> Result<DatasetMetadata> {
        send_and_unwrap_json(self)
    }

    fn fmt_prefix(&self) -> Option<String> {
        Some(format!("/datasets/{}/{}/metadata.json", self.database_code, self.dataset_code))
    }

    fn fmt_arguments(&self) -> Option<String> {
        ApiParameters::fmt(self)
    }
}

impl ApiCall<DatabaseList> for DatabaseSearch {
    fn fmt_prefix(&self) -> Option<String> {
        Some(String::from("/databases.json"))
    }

    fn fmt_arguments(&self) -> Option<String> {
        let arg_1 = ApiParameters::fmt(self);
        let arg_2 = SearchParameters::fmt(self);

        if arg_1.is_some() && arg_2.is_some() {
            Some(format!("{}&{}", arg_1.unwrap(), arg_2.unwrap()))
        } else if arg_1.is_some() {
            arg_1
        } else if arg_2.is_some() {
            arg_2
        } else {
            None
        }
    }
}

impl ApiCall<DatasetList> for DatasetSearch {
    fn fmt_prefix(&self) -> Option<String> {
        Some(String::from("/datasets.json"))
    }

    fn fmt_arguments(&self) -> Option<String> {
        let arg_1 = ApiParameters::fmt(self);
        let arg_2 = SearchParameters::fmt(self);

        if arg_1.is_some() && arg_2.is_some() {
            Some(format!("{}&{}&database_code={}", arg_1.unwrap(),
                                                   arg_2.unwrap(),
                                                   self.database_code))
        } else if arg_1.is_some() {
            Some(format!("{}&database_code={}", arg_1.unwrap(), self.database_code))
        } else if arg_2.is_some() {
            Some(format!("{}&database_code={}", arg_2.unwrap(), self.database_code))
        } else {
            None
        }
    }
}

impl ApiCall<Vec<Code>> for CodeListQuery {
    fn send(&self) -> Result<Vec<Code>> {
        use csv;
        use zip::read::ZipArchive;
        use std::io::{Cursor, Read};

        let zipped_data = try!(self.encoded_data());

        match ZipArchive::new(Cursor::new(zipped_data)) {
            Ok(mut files) => {
                let csv = {
                    let mut csv = String::new();

                    for index in 0..files.len() {
                        if let Err(e) = files.by_index(index).unwrap().read_to_string(&mut csv) {
                            return Err(Error::ParsingFailed(e.to_string()));
                        }
                    }

                    csv
                };

                let mut reader = csv::Reader::from_string(csv);
                let mut codes: Vec<Code> = vec![];

                for record in reader.decode() {
                    let record: (String, String) = {
                        match record {
                            Ok(record) => record,
                            Err(e) => return Err(Error::ParsingFailed(e.to_string())),
                        }
                    };

                    let (database_code, dataset_code) = {
                        let pair: Vec<_> = record.0.split('/').collect();

                        if pair.len() != 2 {
                            let error_message = {
                                "Invalid format for dataset codes in unzipped code list."
                            };

                            return Err(Error::ParsingFailed(error_message.to_string()));
                        }

                        (pair[0].to_string(), pair[1].to_string())
                    };

                    codes.push(Code {
                        database_code: database_code,
                        dataset_code: dataset_code,
                        name: record.1,
                    });
                }

                Ok(codes)
            },

            Err(e) => Err(Error::ParsingFailed(e.to_string())),
        }
    }

    fn fmt_prefix(&self) -> Option<String> {
        Some(format!("/databases/{}/codes", self.database_code))
    }

    fn fmt_arguments(&self) -> Option<String> {
        ApiParameters::fmt(self)
    }
}

impl<T: Decodable + Clone> ApiCall<Data<T>> for DataQuery {
    fn send(&self) -> Result<Data<T>> {
        send_and_unwrap_json(self)
    }

    fn fmt_prefix(&self) -> Option<String> {
        Some(format!("/datasets/{}/{}/data.json", self.database_code, self.dataset_code))
    }

    fn fmt_arguments(&self) -> Option<String> {
        let arg_1 = ApiParameters::fmt(self);
        let arg_2 = DataParameters::fmt(self);

        if arg_1.is_some() && arg_2.is_some() {
            Some(format!("{}&{}", arg_1.unwrap(), arg_2.unwrap()))
        } else if arg_1.is_some() {
            arg_1
        } else if arg_2.is_some() {
            arg_2
        } else {
            None
        }
    }
}

impl<T: Decodable + Clone> ApiCall<DataAndMetadata<T>> for DataAndMetadataQuery {
    fn send(&self) -> Result<DataAndMetadata<T>> {
        send_and_unwrap_json(self)
    }

    fn fmt_prefix(&self) -> Option<String> {
        Some(format!("/datasets/{}/{}.json", self.database_code, self.dataset_code))
    }

    fn fmt_arguments(&self) -> Option<String> {
        let arg_1 = ApiParameters::fmt(self);
        let arg_2 = DataParameters::fmt(self);

        if arg_1.is_some() && arg_2.is_some() {
            Some(format!("{}&{}", arg_1.unwrap(), arg_2.unwrap()))
        } else if arg_1.is_some() {
            arg_1
        } else if arg_2.is_some() {
            arg_2
        } else {
            None
        }
    }
}

impl ApiParameters for DatabaseSearch {}
impl ApiParameters for DatasetSearch {}
impl ApiParameters for DatabaseMetadataQuery {}
impl ApiParameters for DatasetMetadataQuery {}
impl ApiParameters for CodeListQuery {}
impl ApiParameters for DataQuery {}
impl ApiParameters for DataAndMetadataQuery {}
impl SearchParameters for DatabaseSearch {}
impl SearchParameters for DatasetSearch {}
impl DataParameters for DataQuery {}
impl DataParameters for DataAndMetadataQuery {}

impl_has!(DatabaseSearch, ApiArguments, request_arguments);
impl_has!(DatabaseSearch, SearchArguments, search_arguments);
impl_has!(DatasetSearch, ApiArguments, request_arguments);
impl_has!(DatasetSearch, SearchArguments, search_arguments);
impl_has!(DatabaseMetadataQuery, ApiArguments, request_arguments);
impl_has!(DatasetMetadataQuery, ApiArguments, request_arguments);
impl_has!(CodeListQuery, ApiArguments, request_arguments);
impl_has!(DataQuery, DataArguments, data_arguments);
impl_has!(DataQuery, ApiArguments, request_arguments);
impl_has!(DataAndMetadataQuery, DataArguments, data_arguments);
impl_has!(DataAndMetadataQuery, ApiArguments, request_arguments);

fn send_and_unwrap_json<T: Decodable + Clone, A: ApiCall<T>>(api_call: &A) -> Result<T> {
    let json_data = {
        let data = try!(ApiCall::<T>::encoded_data(api_call));

        match String::from_utf8(data) {
            Ok(data) => data,
            Err(e)   => return Err(Error::ParsingFailed(e.to_string())),
        }
    };

    match json::decode::<BTreeMap<String, T>>(&json_data[..]) {
        Ok(tree) => {
            if tree.len() == 1 {
                Ok(tree.iter().next().unwrap().1.clone())
            } else {
                Err(Error::ParsingFailed(format!("Expected a single element, got {}.",
                                                 tree.len())))
            }
        },

        Err(e)   => Err(Error::ParsingFailed(e.to_string())),
    }
}