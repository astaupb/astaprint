use std::{
    io,
    process::{
        Child, Command, Stdio, 
    },
};

pub fn pdfjam(arguments: &[&str]) -> io::Result<Child>
{
    Command::new("pdfjam")
        .args(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

pub fn gs(arguments: &[&str]) -> io::Result<Child>
{
    Command::new("gs")
        .args(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

pub fn gs_inkcov(path: &str) -> io::Result<Child>
{
    gs(&["-dSAFER", "-dBATCH", "-dNOPAUSE", "-dNumRenderingThreads=4", "-sDEVICE=inkcov", "-o", "-", path])
}

pub fn gs_preprocess(path: &str, out: &str) -> io::Result<Child>
{
    gs(&[
        "-dSAFER",
        "-dBATCH",
        "-dNOPAUSE",
        "-dQUIET",
        "-dFitPage",
        "-dColorScreen",
        "-sDEVICE=pdfwrite",
        "-dCompatibilityLevel=1.4",
        "-dPrinted",
        "-dNumRenderingThreads=4",
        &format!("-sOutputFile={}", out),
        &path,
    ])
}

pub fn gs_jpeg(path: &str) -> io::Result<Child>
{
    gs(&[
        "-dSAFER",
        "-dBATCH",
        "-dNOPAUSE",
        "-dQUIET",
        "-sDEVICE=png16m",
        "-dNumRenderingThreads=4",
        "-o",
        &format!("{}%03d", path),
        "-r300",
        &path,
    ])
}

pub fn img2pdf(path: &str, out: &str) -> io::Result<Child>
{
    Command::new("img2pdf")
        .args(&[path, "-o", out])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

pub fn qpdf(arguments: &[&str]) -> io::Result<Child>
{
    Command::new("qpdf")
        .args(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

pub fn qpdf_decrypt(path: &str, out: &str, password: &str) -> io::Result<Child>
{
    qpdf(&["--decrypt", &format!("--password={}", password), &path, &out])
}

pub fn qpdf_pages(path: &str, out: &str, pages: &str) -> io::Result<Child>
{
    qpdf(&["--empty", "--pages", path, pages, "--", out])
}

pub fn qpdf_rotate(path: &str, out: &str, angle: &str, pages: &str) -> io::Result<Child>
{
    qpdf(&[&format!("--rotate={}:{}", angle, pages), path, out])
}

pub fn qpdf_merge(input: &[String], out: &str) -> io::Result<Child>
{
    let mut args = vec!["--empty", "--pages"];
    for page in input {
        args.extend_from_slice(&[&page, "1-z"]);
    }
    args.extend_from_slice(&["--", out]);
    qpdf(&args[..])
}

pub fn qpdf_force_version(path: &str, out: &str) -> io::Result<Child>
{
    qpdf(&["--force-version=1.4", "--object-streams=disable", path, out])
}
