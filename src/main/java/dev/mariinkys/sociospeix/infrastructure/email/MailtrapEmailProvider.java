package dev.mariinkys.sociospeix.infrastructure.email;

import dev.mariinkys.sociospeix.application.exception.EmailSendException;
import dev.mariinkys.sociospeix.application.port.EmailPort;
import dev.mariinkys.sociospeix.domain.model.EmailAttachment;
import dev.mariinkys.sociospeix.infrastructure.config.EmailProperties;
import io.mailtrap.client.MailtrapClient;
import io.mailtrap.config.MailtrapConfig;
import io.mailtrap.factory.MailtrapClientFactory;
import io.mailtrap.model.request.emails.Address;
import io.mailtrap.model.request.emails.MailtrapMail;
import org.springframework.stereotype.Component;

import java.util.Base64;
import java.util.List;

@Component
public class MailtrapEmailProvider implements EmailPort {

    private final MailtrapClient mailtrap;
    private final String from;
    private final int dailyLimit;
    private final boolean configured;

    public MailtrapEmailProvider(EmailProperties properties) {
        var config = properties.forProvider("mailtrap");
        this.configured = config.isConfigured();
        this.from = properties.getFrom();
        this.dailyLimit = config.getDailyLimit();

        if (configured) {
            MailtrapConfig mailtrapConfig = new MailtrapConfig.Builder()
                    .token(config.getApiKey())
                    .build();
            this.mailtrap = MailtrapClientFactory.createMailtrapClient(mailtrapConfig);
        } else {
            this.mailtrap = null;
        }
    }

    @Override
    public void send(String subject, String htmlBody,
                     List<String> recipientEmails,
                     List<EmailAttachment> attachments) {
        if (!configured) {
            throw new EmailSendException("Mailtrap provider is not configured (missing API key)");
        }

        var mailtrapAttachments = attachments.stream()
                .map(a -> {
                    var builder = io.mailtrap.model.request.emails.EmailAttachment.builder()
                            .filename(a.filename())
                            .content(Base64.getEncoder().encodeToString(a.content()));
                    if (a.contentId() != null) {
                        builder.contentId(a.contentId());
                        builder.disposition("inline");
                    }
                    return builder.build();
                })
                .toList();

        var recipients = recipientEmails.stream()
                .map(Address::new)
                .toList();

        var mail = MailtrapMail.builder()
                .from(new Address(from))
                .to(recipients)
                .subject(subject)
                .html(htmlBody)
                .attachments(mailtrapAttachments)
                .build();

        try {
            mailtrap.send(mail);
        } catch (Exception e) {
            throw new EmailSendException("Mailtrap error: " + e.getMessage());
        }
    }

    @Override
    public String getProviderName() { return "MAILTRAP"; }

    @Override
    public int getDailyLimit() { return dailyLimit; }

    @Override
    public boolean isConfigured() { return configured; }
}