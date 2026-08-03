package dev.mariinkys.sociospeix.interfaces.dto.formOcr;

import dev.mariinkys.sociospeix.domain.formOcr.FormOcrResult;
import java.util.List;

public record FormOcrResponse(List<FormOcrResult.FormOcrPage> pages) {
    public FormOcrResponse(FormOcrResult result) {
        this(result.pages());
    }
}