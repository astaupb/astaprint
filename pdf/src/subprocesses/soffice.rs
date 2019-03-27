use crate::tmp::TmpFile;
use std::{
    io,
    process::{
        Command,
        Stdio,
    },
};

const SOFFICE_FORMATS: &[&str] = &[
    "text",
    "Rich Text Format",
    "Composite Document File V2 Document",
    "Microsoft Word 2007+",
    "Microsoft Excel 2007+",
    "Microsoft PowerPoint 2007+",
    "OpenDocument Text",
    "OpenDocument Spreadsheet",
    "OpenDocument Presentation",
];

#[derive(Debug)]
pub enum SOfficeError
{
    IoError(io::Error),
    FormatError,
}

impl From<io::Error> for SOfficeError
{
    fn from(error: io::Error) -> Self { SOfficeError::IoError(error) }
}
pub fn document_to_pdf(data: Vec<u8>) -> Result<Vec<u8>, SOfficeError>
{
    let path = TmpFile::create(&data[..])?;
    let file =
        Command::new("file").arg(&path).stdout(Stdio::piped()).stderr(Stdio::piped()).output()?;

    if SOFFICE_FORMATS
        .iter()
        .all(|format| !(String::from_utf8_lossy(&file.stdout[..]).contains(format)))
    {
        return Err(SOfficeError::FormatError)
    }
    let _soffice = Command::new("soffice")
        .args(&["--headless", "--convert-to", "pdf", "--outdir", "/tmp", &path])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    Ok(TmpFile::remove(&format!("{}.pdf", path))?)
}

#[cfg(test)]
mod tests
{
    use crate::{
        sanitize,
        subprocesses::soffice::document_to_pdf,
    };
    use std::{
        fs::{
            read,
            File,
        },
        io::Read,
        path::Path,
    };
    fn read_and_test(name: &str)
    {
        println!("testing {}", name);
        let mut data = read(name).unwrap();
        let doc = document_to_pdf(data);
        match doc {
            Ok(data) => 
            println!("{}", &String::from_utf8_lossy(&data[..])),
            Err(e) => panic!("{:?}", e),
        }
    }
    #[test]
    fn soffice()
    {
        for entry in Path::new("soffice").read_dir().expect("reading ./soffice") {
            if let Ok(doc) = entry {
                read_and_test(doc.path().to_str().unwrap());
            }
        }
    }
}
