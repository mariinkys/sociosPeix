package dev.mariinkys.sociospeix.application.port;

import dev.mariinkys.sociospeix.domain.model.EmailAttachment;

import java.util.List;

// Interface for implementing new EmailProviders
public interface EmailPort {
    void send(String subject, String htmlBody,
              List<String> recipientEmails,
              List<EmailAttachment> attachments);

    String getProviderName(); // stored in DB so we know which provider sent each email

    int getDailyLimit(); // each provider implementation has to declare this

    boolean isConfigured();
}