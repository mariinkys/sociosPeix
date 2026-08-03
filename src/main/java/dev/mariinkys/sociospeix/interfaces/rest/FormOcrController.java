package dev.mariinkys.sociospeix.interfaces.rest;


import dev.mariinkys.sociospeix.application.port.FormOcrUseCase;
import dev.mariinkys.sociospeix.domain.formOcr.FormOcrResult;
import dev.mariinkys.sociospeix.interfaces.dto.formOcr.FormOcrResponse;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestPart;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.multipart.MultipartFile;

@RestController
@RequestMapping("/api/formOcr")
public class FormOcrController {
    private final FormOcrUseCase formOcrUseCase;
    private static final Logger log = LoggerFactory.getLogger(FormOcrController.class);

    public FormOcrController(FormOcrUseCase formOcrUseCase) {
        this.formOcrUseCase = formOcrUseCase;
    }

    @PostMapping(value = "/analyze", consumes = MediaType.MULTIPART_FORM_DATA_VALUE)
    public ResponseEntity<FormOcrResponse> analyze(
            @RequestPart("file") MultipartFile file) {
        log.info("Form OCR has been requested, forwarding to OCR service");
        FormOcrResult result = formOcrUseCase.analyzeFile(file);
        return ResponseEntity.ok(new FormOcrResponse(result));
    }
}
