package dev.mariinkys.sociospeix.application.port;

import dev.mariinkys.sociospeix.domain.formOcr.FormOcrResult;
import org.springframework.web.multipart.MultipartFile;

public interface FormOcrUseCase {
    FormOcrResult analyzeFile(MultipartFile file);
}
