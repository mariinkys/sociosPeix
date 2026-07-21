package dev.mariinkys.sociospeix.interfaces.dto.email;

import jakarta.validation.constraints.NotBlank;

public record SendEmailRequest(
        @NotBlank(message = "Subject is required")
        String subject,

        @NotBlank(message = "Body is required")
        String htmlBody,

        String previewHtmlBody
) {}
