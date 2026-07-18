package dev.mariinkys.sociospeix.interfaces.dto.email;

import dev.mariinkys.sociospeix.domain.model.Email;
import dev.mariinkys.sociospeix.domain.model.EmailCategory;

import java.time.LocalDateTime;
import java.util.List;
import java.util.UUID;

public record EmailResponse(
        UUID id,
        String subject,
        String provider,
        String body,
        List<String> recipientEmails,
        int recipientCount,
        EmailCategory category,
        LocalDateTime createdAt
) {
    public static EmailResponse from(Email email) {
        return new EmailResponse(
                email.getId(),
                email.getSubject(),
                email.getProvider(),
                email.getBody(),
                email.getRecipientEmails(),
                email.getRecipientEmails().size(),
                email.getCategory(),
                email.getCreatedAt()
        );
    }
}