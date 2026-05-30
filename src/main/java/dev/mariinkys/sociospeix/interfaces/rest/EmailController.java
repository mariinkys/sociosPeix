package dev.mariinkys.sociospeix.interfaces.rest;

import dev.mariinkys.sociospeix.application.port.EmailUseCase;
import dev.mariinkys.sociospeix.domain.model.EmailAttachment;
import dev.mariinkys.sociospeix.interfaces.dto.email.EmailResponse;
import dev.mariinkys.sociospeix.interfaces.dto.PageResponse;
import dev.mariinkys.sociospeix.interfaces.dto.email.SendEmailRequest;
import dev.mariinkys.sociospeix.interfaces.dto.email.SendEmailToInterestsRequest;
import jakarta.validation.Valid;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;
import java.util.List;
import java.util.UUID;

@RestController
@RequestMapping("/api/emails")
public class EmailController {

    private final EmailUseCase emailUseCase;

    public EmailController(EmailUseCase emailUseCase) {
        this.emailUseCase = emailUseCase;
    }

    @GetMapping
    public ResponseEntity<PageResponse<EmailResponse>> getAll(
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size) {

        var pageable = PageRequest.of(page, size, Sort.by(Sort.Direction.DESC, "createdAt"));
        return ResponseEntity.ok(PageResponse.from(
                emailUseCase.getAllEmails(pageable).map(EmailResponse::from)
        ));
    }

    @GetMapping("/{id}")
    public ResponseEntity<EmailResponse> getById(@PathVariable UUID id) {
        return ResponseEntity.ok(EmailResponse.from(emailUseCase.getEmailById(id)));
    }

    @GetMapping("/member/{memberId}")
    public ResponseEntity<PageResponse<EmailResponse>> getByMember(
            @PathVariable UUID memberId,
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size) {

        var pageable = PageRequest.of(page, size, Sort.by(Sort.Direction.DESC, "createdAt"));
        return ResponseEntity.ok(PageResponse.from(
                emailUseCase.getAllEmailsByMember(memberId, pageable).map(EmailResponse::from)
        ));
    }

    // Send to a specific member
    @PostMapping(value = "/send/member/{memberId}", consumes = MediaType.MULTIPART_FORM_DATA_VALUE)
    public ResponseEntity<EmailResponse> sendToMember(
            @PathVariable UUID memberId,
            @RequestPart("data") @Valid SendEmailRequest request,
            @RequestPart(value = "attachments", required = false) List<MultipartFile> attachments) {

        var email = emailUseCase.sendToMember(
                memberId, request.subject(), request.htmlBody(), toAttachments(attachments)
        );
        return ResponseEntity.status(HttpStatus.CREATED).body(EmailResponse.from(email));
    }

    // Send to all members
    @PostMapping(value = "/send/all", consumes = MediaType.MULTIPART_FORM_DATA_VALUE)
    public ResponseEntity<EmailResponse> sendToAll(
            @RequestPart("data") @Valid SendEmailRequest request,
            @RequestPart(value = "attachments", required = false) List<MultipartFile> attachments) {

        var email = emailUseCase.sendToAll(
                request.subject(), request.htmlBody(), toAttachments(attachments)
        );
        return ResponseEntity.status(HttpStatus.CREATED).body(EmailResponse.from(email));
    }

    // Send to members filtered by interests (OR — any of the given interests)
    @PostMapping(value = "/send/interests", consumes = MediaType.MULTIPART_FORM_DATA_VALUE)
    public ResponseEntity<EmailResponse> sendToInterests(
            @RequestPart("data") @Valid SendEmailToInterestsRequest request,
            @RequestPart(value = "attachments", required = false) List<MultipartFile> attachments) {

        var email = emailUseCase.sendToInterests(
                request.interestIds(), request.subject(), request.htmlBody(), toAttachments(attachments)
        );
        return ResponseEntity.status(HttpStatus.CREATED).body(EmailResponse.from(email));
    }

    // Converts Spring MultipartFile → domain EmailAttachment
    private List<EmailAttachment> toAttachments(List<MultipartFile> files) {
        if (files == null) return List.of();
        return files.stream()
                .map(file -> {
                    try {
                        return new EmailAttachment(
                                file.getOriginalFilename(),
                                file.getContentType(),
                                file.getBytes()
                        );
                    } catch (IOException e) {
                        throw new IllegalArgumentException(
                                "Failed to read attachment: " + file.getOriginalFilename()
                        );
                    }
                })
                .toList();
    }
}