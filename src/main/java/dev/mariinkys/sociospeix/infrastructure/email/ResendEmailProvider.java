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
    private final int dailyLimit;
    private final boolean configured;

    public ResendEmailProvider(EmailProperties properties) {
        var config = properties.forProvider("resend");
        this.configured = config.isConfigured();
        this.from = properties.getFrom();
        this.dailyLimit = config.getDailyLimit();

        this.resend = configured ? new Resend(config.getApiKey()) : null;
    }

    @Override
    public void send(String subject, String htmlBody,
                     List<String> recipientEmails,
                     List<EmailAttachment> attachments) {

        if (!configured) {
            throw new EmailSendException("Resend provider is not configured (missing API key)");
        }

        var resendAttachments = attachments.stream()
                .map(a -> {
                    var builder = Attachment.builder()
                            .fileName(a.filename())
                            .content(Base64.getEncoder().encodeToString(a.content()));

                    if (a.contentId() != null) {
                        builder.contentId(a.contentId());
                    }

                    return builder.build();
                })
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
    public String getProviderName() { return "RESEND"; }

    @Override
    public int getDailyLimit() { return dailyLimit; }

    @Override
    public boolean isConfigured() { return configured; }
}