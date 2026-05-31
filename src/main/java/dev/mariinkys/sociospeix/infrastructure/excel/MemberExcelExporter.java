package dev.mariinkys.sociospeix.infrastructure.excel;

import dev.mariinkys.sociospeix.application.port.ExcelExporter;
import dev.mariinkys.sociospeix.domain.model.Interest;
import dev.mariinkys.sociospeix.domain.model.Member;
import org.apache.poi.ss.usermodel.*;
import org.apache.poi.xssf.usermodel.XSSFWorkbook;
import org.springframework.stereotype.Component;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.List;

@Component
public class MemberExcelExporter implements ExcelExporter<Member> {

    private static final String[] HEADERS = {
            "ID", "Name", "Surname", "Second Surname", "Email",
            "Birthdate", "Phone", "Gender", "Country", "Interests", "Notes"
    };

    @Override
    public byte[] export(List<Member> members) {
        try (var workbook = new XSSFWorkbook();
             var out = new ByteArrayOutputStream()) {

            var sheet = workbook.createSheet("Members");
            createHeaderRow(workbook, sheet);

            int rowIndex = 1;
            for (Member member : members) {
                createDataRow(sheet, rowIndex++, member);
            }

            // Auto-size all columns
            for (int i = 0; i < HEADERS.length; i++) {
                sheet.autoSizeColumn(i);
            }

            workbook.write(out);
            return out.toByteArray();

        } catch (IOException e) {
            throw new RuntimeException("Failed to generate Excel file", e);
        }
    }

    @Override
    public String getFileName() {
        return "members.xlsx";
    }

    private void createHeaderRow(Workbook workbook, Sheet sheet) {
        var headerStyle = workbook.createCellStyle();
        var font = workbook.createFont();
        font.setBold(true);
        headerStyle.setFont(font);
        headerStyle.setFillForegroundColor(IndexedColors.GREY_25_PERCENT.getIndex());
        headerStyle.setFillPattern(FillPatternType.SOLID_FOREGROUND);

        var row = sheet.createRow(0);
        for (int i = 0; i < HEADERS.length; i++) {
            var cell = row.createCell(i);
            cell.setCellValue(HEADERS[i]);
            cell.setCellStyle(headerStyle);
        }
    }

    private void createDataRow(Sheet sheet, int rowIndex, Member member) {
        var row = sheet.createRow(rowIndex);

        row.createCell(0).setCellValue(member.getId().toString() != null ? member.getId().toString() : "");
        row.createCell(1).setCellValue(member.getName());
        row.createCell(2).setCellValue(member.getSurname());
        row.createCell(3).setCellValue(member.getSecondSurname());
        row.createCell(4).setCellValue(member.getEmail());
        row.createCell(5).setCellValue(
                member.getBirthdate() != null ? member.getBirthdate().toString() : ""
        );
        row.createCell(6).setCellValue(member.getPhone());
        row.createCell(7).setCellValue(
                member.getGender() != null ? member.getGender().getName() : ""
        );
        row.createCell(8).setCellValue(
                member.getCountry() != null ? member.getCountry().getName() : ""
        );
        row.createCell(9).setCellValue(
                member.getInterests().stream()
                        .map(Interest::getName)
                        .collect(java.util.stream.Collectors.joining(", "))
        );
        row.createCell(10).setCellValue(member.getNotes());
    }
}