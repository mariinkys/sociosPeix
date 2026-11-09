//! Spreadsheet export via `rust_xlsxwriter`.

use rust_xlsxwriter::{Color, Format, FormatPattern, Workbook, Worksheet, XlsxError};

use crate::{
    app_error::{AppError, AppResult},
    entities::{member::MemberExportRow, statistics::InterestPopularity},
    use_cases::member::SpreadsheetExporter,
};

#[derive(Default)]
pub struct XlsxExporter;

const MEMBER_HEADERS: [&str; 11] = [
    "ID",
    "Name",
    "Surname",
    "Second Surname",
    "Email",
    "Birthdate",
    "Phone",
    "Gender",
    "Country",
    "Interests",
    "Notes",
];

const POPULARITY_HEADERS: [&str; 2] = ["Interest", "Member Count"];

fn header_format() -> Format {
    Format::new()
        .set_bold()
        .set_pattern(FormatPattern::Solid)
        .set_background_color(Color::RGB(0xC0C0C0))
}

fn write_headers(sheet: &mut Worksheet, headers: &[&str]) -> Result<(), XlsxError> {
    let format = header_format();
    for (col, header) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, col as u16, *header, &format)?;
    }
    sheet.set_freeze_panes(1, 0)?;
    Ok(())
}

fn finish(mut workbook: Workbook) -> AppResult<Vec<u8>> {
    workbook.save_to_buffer().map_err(AppError::internal)
}

impl SpreadsheetExporter for XlsxExporter {
    fn members(&self, rows: &[MemberExportRow]) -> AppResult<Vec<u8>> {
        let build = || -> Result<Workbook, XlsxError> {
            let mut workbook = Workbook::new();
            let sheet = workbook.add_worksheet();
            sheet.set_name("Members")?;
            write_headers(sheet, &MEMBER_HEADERS)?;
            for (i, m) in rows.iter().enumerate() {
                let r = i as u32 + 1;
                sheet.write_string(r, 0, m.id.to_string())?;
                sheet.write_string(r, 1, &m.name)?;
                sheet.write_string(r, 2, &m.surname)?;
                sheet.write_string(r, 3, &m.second_surname)?;
                sheet.write_string(r, 4, &m.email)?;
                sheet.write_string(r, 5, m.birthdate.map(|d| d.to_string()).unwrap_or_default())?;
                sheet.write_string(r, 6, &m.phone)?;
                sheet.write_string(r, 7, m.gender.as_deref().unwrap_or_default())?;
                sheet.write_string(r, 8, m.country.as_deref().unwrap_or_default())?;
                sheet.write_string(r, 9, m.interests.join(", "))?;
                sheet.write_string(r, 10, &m.notes)?;
            }
            sheet.autofit();
            Ok(workbook)
        };
        finish(build().map_err(AppError::internal)?)
    }

    fn interest_popularity(&self, rows: &[InterestPopularity]) -> AppResult<Vec<u8>> {
        let build = || -> Result<Workbook, XlsxError> {
            let mut workbook = Workbook::new();
            let sheet = workbook.add_worksheet();
            sheet.set_name("Interest Popularity")?;
            write_headers(sheet, &POPULARITY_HEADERS)?;
            for (i, row) in rows.iter().enumerate() {
                let r = i as u32 + 1;
                sheet.write_string(r, 0, &row.interest_name)?;
                sheet.write_number(r, 1, row.member_count as f64)?;
            }
            sheet.autofit();
            Ok(workbook)
        };
        finish(build().map_err(AppError::internal)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_valid_xlsx_zip() {
        let bytes = XlsxExporter
            .interest_popularity(&[InterestPopularity {
                interest_name: "Golf".into(),
                member_count: 3,
            }])
            .unwrap();
        assert_eq!(&bytes[..2], b"PK");
        let bytes = XlsxExporter.members(&[]).unwrap();
        assert_eq!(&bytes[..2], b"PK");
    }
}
