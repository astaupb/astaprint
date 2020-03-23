use std::{
    io::{self, Read},
    fs::File,
};

/// contains all the information needed to talk to the printer
#[derive(Debug, Clone)]
pub struct PPD
{
    pub begin: Vec<u8>,
    pub to_pdf: Vec<u8>,
    pub end: Vec<u8>,
    pub page_size_a3: Vec<u8>,
    pub page_size_a4: Vec<u8>,
    pub page_region_a3: Vec<u8>,
    pub page_region_a4: Vec<u8>,
    pub tray_bypass: Vec<u8>,
    pub tray_auto: Vec<u8>,
    pub duplex_off: Vec<u8>,
    pub duplex_long: Vec<u8>,
    pub duplex_short: Vec<u8>,
    pub copies: Vec<u8>,
    pub copies_collate: Vec<u8>,
    pub color: Vec<u8>,
    pub greyscale: Vec<u8>,
}

/// parses the pjl from a line of the ppd file
pub fn parse_pjl(line: &str) -> Vec<u8>
{
    line[line.find('\"').unwrap() + 1 .. line.rfind('\"')
        .unwrap()]
        .replace("<0A>", "\x0a")
        .replace("<1B>", "\x1b")
        .as_bytes()
        .to_owned()
}

impl PPD
{
    pub fn new_from_file(path: &str) -> io::Result<PPD>
    {
        let mut begin = Vec::new();
        let mut to_pdf = Vec::new();
        let mut end = Vec::new();
        let mut page_size_a3 = Vec::new();
        let mut page_size_a4 = Vec::new();
        let mut page_region_a3 = Vec::new();
        let mut page_region_a4 = Vec::new();
        let mut tray_bypass = Vec::new();
        let mut tray_auto = Vec::new();
        let mut duplex_off = Vec::new();
        let mut duplex_long = Vec::new();
        let mut duplex_short = Vec::new();
        let mut copies = Vec::new();
        let mut copies_collate = Vec::new();
        let mut color = Vec::new();
        let mut greyscale = Vec::new();

        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;
        for line in contents.lines() {
            if line.starts_with("*JCLBegin") {
                begin = parse_pjl(line);
            } else if line.starts_with("*JCLToPDFInterpreter") {
                to_pdf = parse_pjl(line);
            } else if line.starts_with("*JCLEnd") {
                end = parse_pjl(line);
            } else if line.starts_with("*PageSize A3") {
                page_size_a3 = parse_pjl(line);
            } else if line.starts_with("*PageSize A4") {
                page_size_a4 = parse_pjl(line);
            } else if line.starts_with("*PageRegion A3") {
                page_region_a3 = parse_pjl(line);
            } else if line.starts_with("*PageRegion A4") {
                page_region_a4 = parse_pjl(line);
            } else if line.starts_with("*InputSlot MultiTray") {
                tray_bypass = parse_pjl(line);
            } else if line.starts_with("*InputSlot Auto") {
                tray_auto = parse_pjl(line);
            } else if line.starts_with("*Duplex None") {
                duplex_off = parse_pjl(line);
            } else if line.starts_with("*Duplex DuplexNoTumble") {
                duplex_long = parse_pjl(line);
            } else if line.starts_with("*Duplex DuplexTumble") {
                duplex_short = parse_pjl(line);
            } else if line.starts_with("*Collate False") {
                copies = parse_pjl(line);
            } else if line.starts_with("*Collate True") {
                copies_collate = parse_pjl(line);
            } else if line.starts_with("*ColorModel CMYK") {
                color = parse_pjl(line);
            } else if line.starts_with("*ColorModel Gray") {
                greyscale = parse_pjl(line);
            }
        }
        Ok(PPD{
            begin,
            to_pdf,
            end,
            page_size_a3,
            page_size_a4,
            page_region_a3,
            page_region_a4,
            tray_bypass,
            tray_auto,
            duplex_off,
            duplex_long,
            duplex_short,
            copies,
            copies_collate,
            color,
            greyscale,
        })
    }
}

#[cfg(test)]
mod tests
{
    use crate::ppd::PPD;
    #[test]
    fn test_parse_ppd_file()
    {
        let ppd = PPD::new_from_file("./Ricoh-MP_C4504ex-PDF-Ricoh.ppd").expect("creating PPD");
        println!("{:?}", ppd);
    }
}
