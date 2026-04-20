use crate::core::{Error, GachaRecord, Result};
use rust_xlsxwriter::{Format, Workbook};
use std::{io::Write, path::Path};

const HEADERS: [&str; 5] = ["时间", "名称", "星级", "卡池类型", "版本"];

pub fn export_csv(records: &[GachaRecord], writer: impl Write) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(writer);
    wtr.write_record(HEADERS)?;
    for record in records {
        wtr.write_record([
            record.time.to_string(),
            record.name.clone(),
            record.quality_level.to_string(),
            record.card_pool.to_string(),
            record.version.clone(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn export_xlsx(records: &[GachaRecord], path: &str) -> Result<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let header_fmt = Format::new().set_bold();
    for (col, header) in HEADERS.iter().enumerate() {
        worksheet.write_with_format(0, col as u16, *header, &header_fmt)?;
    }

    for (i, record) in records.iter().enumerate() {
        let row = (i + 1) as u32;
        worksheet.write_string(row, 0, record.time.to_string())?;
        worksheet.write_string(row, 1, &record.name)?;
        worksheet.write_string(row, 2, record.quality_level.to_string())?;
        worksheet.write_string(row, 3, record.card_pool.to_string())?;
        worksheet.write_string(row, 4, &record.version)?;
    }

    worksheet.set_column_width(0, 20)?;
    worksheet.set_column_width(1, 16)?;
    worksheet.set_column_width(2, 8)?;
    worksheet.set_column_width(3, 12)?;
    worksheet.set_column_width(4, 8)?;

    workbook.save(path)?;
    Ok(())
}

pub fn export_json(records: &[GachaRecord], writer: impl Write) -> Result<()> {
    serde_json::to_writer_pretty(writer, records)?;
    Ok(())
}

/// Detect format from file extension and export accordingly.
pub fn export_to_file(records: &[GachaRecord], path: &str) -> Result<()> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext.to_ascii_lowercase().as_str() {
        "csv" => {
            let file = std::fs::File::create(path)?;
            export_csv(records, file)
        }
        "xlsx" => export_xlsx(records, path),
        "json" => {
            let file = std::fs::File::create(path)?;
            export_json(records, file)
        }
        _ => Err(Error::Other(format!("Unsupported export format: .{ext}"))),
    }
}
