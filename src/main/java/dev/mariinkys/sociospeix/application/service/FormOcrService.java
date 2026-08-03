package dev.mariinkys.sociospeix.application.service;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import dev.mariinkys.sociospeix.application.port.FormOcrUseCase;
import dev.mariinkys.sociospeix.domain.formOcr.FormOcrResult;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.core.io.ByteArrayResource;
import org.springframework.http.*;
import org.springframework.stereotype.Service;
import org.springframework.util.LinkedMultiValueMap;
import org.springframework.util.MultiValueMap;
import org.springframework.web.client.RestTemplate;
import org.springframework.web.multipart.MultipartFile;

import java.util.List;

@Service
public class FormOcrService implements FormOcrUseCase {

    private final RestTemplate restTemplate = new RestTemplate();
    private final ObjectMapper objectMapper = new ObjectMapper();

    @Value("${app.ocr.service-url}/extract")
    private String ocrUrl;

    @Override
    public FormOcrResult analyzeFile(MultipartFile file) {
        try {
            HttpHeaders headers = new HttpHeaders();
            headers.setContentType(MediaType.MULTIPART_FORM_DATA);

            ByteArrayResource pdf = new ByteArrayResource(file.getBytes()) {
                @Override
                public String getFilename() {
                    return file.getOriginalFilename() != null ? file.getOriginalFilename() : "upload.pdf";
                }
            };

            MultiValueMap<String, Object> body = new LinkedMultiValueMap<>();
            body.add("file", pdf);

            ResponseEntity<String> response = restTemplate.exchange(
                    ocrUrl,
                    HttpMethod.POST,
                    new HttpEntity<>(body, headers),
                    String.class
            );

            List<FormOcrResult.FormOcrPage> pages = objectMapper.readValue(
                    response.getBody(),
                    new TypeReference<>() {}
            );

            return new FormOcrResult(pages);

        } catch (Exception e) {
            throw new RuntimeException("OCR service call failed: " + e.getMessage(), e);
        }
    }
}