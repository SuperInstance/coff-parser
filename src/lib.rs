//! COFF (Common Object File Format) parser

pub struct CoffHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

pub struct SectionHeader {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
}

pub fn parse_coff_header(_data: &[u8]) -> Result<CoffHeader, String> {
    Ok(CoffHeader {
        machine: 0,
        number_of_sections: 0,
        time_date_stamp: 0,
        pointer_to_symbol_table: 0,
        number_of_symbols: 0,
        size_of_optional_header: 0,
        characteristics: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() { parse_coff_header(&[]).unwrap_err(); }
}
