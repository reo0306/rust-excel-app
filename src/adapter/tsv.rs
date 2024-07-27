//use std::fs::File;
//use std::io::BufReader;
use csv::WriterBuilder;

use crate::domain::model::excel::Record;

const OUTPUT_FILE_PATH: &str = "files/output/test.tsv";

pub struct Tsv;

impl Tsv {
    pub fn write_to_tsv(&self, excel: &Vec<Record>) -> Result<(), Box<dyn std::error::Error>> {
        let mut tsv = WriterBuilder::new().has_headers(false).delimiter(b'\t').from_path(OUTPUT_FILE_PATH)?;

        for record in excel {
            tsv.serialize(record)?;
        };
        
        Ok(())
    }
}

#[cfg(test)]
mod tsv_test {
    use claim::*;

    use super::*;
    use crate::domain::model::excel::Record;

    #[test]
    fn it_file_output() {
        let record = vec![
            Record {
                no: 1,
                product: "test".to_string(),
                price: 100,
                tax_price_eight: 108,
                tax_price_ten: 110,
            }
        ];

        let r = Tsv.write_to_tsv(&record);

        assert_ok!(r);
    }
}