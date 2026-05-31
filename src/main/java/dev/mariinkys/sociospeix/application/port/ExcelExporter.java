package dev.mariinkys.sociospeix.application.port;

import java.util.List;

public interface ExcelExporter<T> {
    byte[] export(List<T> data);
    String getFileName();
}