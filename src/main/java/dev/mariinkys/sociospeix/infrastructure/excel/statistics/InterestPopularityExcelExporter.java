package dev.mariinkys.sociospeix.infrastructure.excel.statistics;

import dev.mariinkys.sociospeix.application.port.ExcelExporter;
import dev.mariinkys.sociospeix.domain.model.statistics.InterestPopularity;
import org.apache.poi.ss.usermodel.*;
import org.apache.poi.xssf.usermodel.XSSFWorkbook;
import org.springframework.stereotype.Component;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.List;

@Component
public class InterestPopularityExcelExporter implements ExcelExporter<InterestPopularity> {

    private static final String[] HEADERS = { "Interest", "Member Count" };

    @Override
    public byte[] export(List<InterestPopularity> rows) {
        try (var workbook = new XSSFWorkbook();
             var out = new ByteArrayOutputStream()) {

            var sheet = workbook.createSheet("Interest Popularity");
            createHeaderRow(workbook, sheet);

            int rowIndex = 1;
            for (InterestPopularity row : rows) {
                var excelRow = sheet.createRow(rowIndex++);
                excelRow.createCell(0).setCellValue(row.interestName());
                excelRow.createCell(1).setCellValue(row.memberCount());
            }

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
        return "interest-popularity.xlsx";
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
}
