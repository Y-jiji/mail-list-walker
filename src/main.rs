#![feature(iterator_try_collect)]
use calamine::{open_workbook_auto, Reader};
use std::path::PathBuf;
mod args;
mod conf;
mod mail;
mod text;

fn work() -> Result<(), String> {
    // get arguments
    let args = args::Args::new();
    // get configurations
    let conf = args
        .get(1, format!("a json configuration file"))
        .map_err(Into::<String>::into)?;
    let conf = conf::Config::load(conf).map_err(Into::<String>::into)?;
    // get xlsx path
    let xlsx = args
        .get(2, format!("excel file"))
        .map_err(Into::<String>::into)?;
    let xlsx = PathBuf::from(xlsx);
    match xlsx.extension().and_then(|s| s.to_str()) {
        Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => (),
        _ => Err("Expecting an excel file as the first argument, but we get {path:?} instead")?,
    }
    // which sheet to send?
    let sheet = args
        .get(3, format!("A sheet name should be provided as the third argument"))
        .map_err(Into::<String>::into)?;
    let mut excel = open_workbook_auto(&xlsx).map_err(|e| e.to_string())?;
    let range = match excel.worksheet_range(&sheet) {
        None => Err(format!("There is no sheet called \"{sheet}\""))?,
        Some(Err(e)) => Err(format!("We suffered {e:?} when we try to get sheet \"{sheet}\""))?,
        Some(Ok(x)) => x,
    };
    // initialize mail client and template from configuration
    let provider = mail::MailService::load(&conf).map_err(|e| e.to_string())?;
    let template = text::MailTemplate::load(&conf).map_err(|e| e.to_string())?;
    // for each row, send a mail use the template
    let n = range.rows().len();
    if n == 0 {
        Err(format!("\"{sheet}\" in {xlsx:?} has 0 rows, which means it doesnt't even have a header!"))?;
    }
    let n = n - 1;
    let mut table_hd = Vec::new();
    let mut all_healthy = true;
    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            table_hd.extend(row);
        } else {
            let info = row.iter().enumerate().map(|(j, col)| (table_hd[j].to_string(), col.to_string()));
            let mail = template.fill(info.collect());
            match provider.send(mail) {
                Err(e) => {println!("{}", e.to_string()); all_healthy=false},
                Ok(()) => println!("mail [{i}/{n}]")
            }
        }
    }
    if !all_healthy {
        Err(format!("There is at least one error, see above logs. "))
    } else {
        Ok(())
    }
}

fn main() {
    match work() {
        Ok(()) => println!("Mails all sent gracefully (*^_^*)"),
        Err(e) => println!("===\n{e} ~~(ToT)~~\n==="),
    }
}