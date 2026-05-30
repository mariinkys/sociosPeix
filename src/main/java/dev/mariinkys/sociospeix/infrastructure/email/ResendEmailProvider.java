package dev.mariinkys.sociospeix.infrastructure.email;

import com.resend.Resend;
import com.resend.core.exception.ResendException;
import com.resend.services.emails.model.Attachment;
import com.resend.services.emails.model.CreateEmailOptions;
import dev.mariinkys.sociospeix.application.exception.EmailSendException;
import dev.mariinkys.sociospeix.application.port.EmailPort;
import dev.mariinkys.sociospeix.domain.model.EmailAttachment;
import dev.mariinkys.sociospeix.infrastructure.config.EmailProperties;
import org.springframework.stereotype.Component;

import java.util.Base64;
import java.util.List;

@Component
public class ResendEmailProvider implements EmailPort {

    private final Resend resend;
    private final String from;

    public ResendEmailProvider(EmailProperties properties) {
        this.resend = new Resend(properties.getResend().getApiKey());
        this.from = properties.getFrom();
    }

    @Override
    public void send(String subject, String htmlBody,
                     List<String> recipientEmails,
                     List<EmailAttachment> attachments) {

        var resendAttachments = attachments.stream()
                .map(a -> Attachment.builder()
                        .fileName(a.filename())
                        .content(Base64.getEncoder().encodeToString(a.content()))
                        .build())
                .toList();

        var options = CreateEmailOptions.builder()
                .from(from)
                .to(recipientEmails)
                .subject(subject)
                .html(htmlBody)
                .attachments(resendAttachments)
                .build();

        try {
            resend.emails().send(options);
        } catch (ResendException e) {
            throw new EmailSendException("Resend error: " + e.getMessage());
        }
    }

    @Override
    public String getProviderName() {
        return "RESEND";
    }
}