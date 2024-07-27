use std::error::Error;

use excel_app::adapter::{
    excel::Excel,
    tsv::Tsv,
};

fn main() -> Result<(), Box<dyn Error>> {
    let excel = Excel.read_file_data()?;

    Tsv.write_to_tsv(&excel)?;

    Ok(())
}