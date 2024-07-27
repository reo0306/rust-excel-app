use calamine::{
    open_workbook,
    Error,
    Xlsx,
    Reader,
    RangeDeserializerBuilder,
};

use crate::domain::model::excel::Record;

const INPUT_FILE_PATH: &str = "files/input/test.xlsx";
const SHEET_NAME: &str = "Sheet1";
const HEADER: &[&str] = &["No.", "商品名", "価格", "税込み(持ち帰り）", "税込み(店内)"];

pub struct Excel;

impl Excel {
    pub fn read_file_data(&self) -> Result<Vec<Record>, Error> {
        let mut workbook: Xlsx<_> = open_workbook(INPUT_FILE_PATH)?;
        let range = workbook.worksheet_range(SHEET_NAME)?;

        let iter_record = RangeDeserializerBuilder::with_headers(HEADER).from_range(&range)?;

        let mut excel = Vec::new();

        for result in iter_record {
            let mut record: Record = result?;
            record.sum = Some(record.price + 100);
            excel.push(record);
        }

        Ok(excel)
    }
}

#[cfg(test)]
mod test_excel {
    use claim::*;
    use super::*;

    #[test]
    fn it_read_file() {
        let excel = Excel.read_file_data();

        assert_ok!(excel);
    }

    #[test]
    fn it_read_file_data() {
        let excel = Excel.read_file_data();
        let data = excel.unwrap();

        assert_eq!(50, data.len());
    }
}