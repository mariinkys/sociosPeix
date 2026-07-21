package dev.mariinkys.sociospeix.interfaces.dto.email;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotEmpty;
import java.util.List;

public record SendEmailToInterestsRequest(
        @NotBlank(message = "Subject is required")
        String subject,

        @NotBlank(message = "Body is required")
        String htmlBody,

        @NotEmpty(message = "At least one interest must be specified")
        List<Integer> interestIds,

        String previewHtmlBody
) {}
