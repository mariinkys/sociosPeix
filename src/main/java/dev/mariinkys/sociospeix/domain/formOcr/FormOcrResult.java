package dev.mariinkys.sociospeix.domain.formOcr;

import java.util.List;
import java.util.Map;

public record FormOcrResult(List<FormOcrPage> pages) {

    public record FormOcrPage(int page, Map<String, FormOcrField> fields) {
    }

    public record FormOcrField(String text, List<String> lines, Double confidence, boolean empty) {
    }
}