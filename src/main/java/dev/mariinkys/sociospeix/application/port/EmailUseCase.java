package dev.mariinkys.sociospeix.application.port;

import dev.mariinkys.sociospeix.application.service.EmailProviderStatus;
import dev.mariinkys.sociospeix.domain.model.Email;
import dev.mariinkys.sociospeix.domain.model.EmailAttachment;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.List;
import java.util.UUID;

public interface EmailUseCase {
    Email sendToMember(UUID memberId, String subject, String htmlBody,
                       List<EmailAttachment> attachments);
    Email sendToAll(String subject, String htmlBody,
                    List<EmailAttachment> attachments);
    Email sendToInterests(List<Integer> interestIds, String subject, String htmlBody,
                          List<EmailAttachment> attachments);
    Page<Email> getAllEmails(Pageable pageable);
    List<Email> getTodayEmails();
    Page<Email> getAllEmailsByMember(UUID memberId, Pageable pageable);
    Email getEmailById(UUID id);
    EmailProviderStatus getProviderStatus();
}