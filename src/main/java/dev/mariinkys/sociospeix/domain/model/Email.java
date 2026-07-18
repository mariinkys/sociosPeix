package dev.mariinkys.sociospeix.domain.model;

import java.time.LocalDateTime;
import java.util.List;
import java.util.UUID;

public class Email {

    private final UUID id;
    private final String subject;
    private final String body;
    private final String provider;
    private final List<String> recipientEmails;
    private final EmailCategory category;
    private final LocalDateTime createdAt;

    // For creating a CAMPAIGN email (default), keeps existing call sites unchanged
    public Email(String subject, String body, String provider, List<String> recipientEmails) {
        this(subject, body, provider, recipientEmails, EmailCategory.CAMPAIGN);
    }

    // For creating an email with an explicit category
    public Email(String subject, String body, String provider,
                 List<String> recipientEmails, EmailCategory category) {
        this.id = null;
        this.subject = subject;
        this.body = body;
        this.provider = provider;
        this.recipientEmails = recipientEmails;
        this.category = category;
        this.createdAt = null;
    }

    // For reconstructing from DB
    public Email(UUID id, String subject, String body, String provider,
                 List<String> recipientEmails, EmailCategory category, LocalDateTime createdAt) {
        this.id = id;
        this.subject = subject;
        this.body = body;
        this.provider = provider;
        this.recipientEmails = recipientEmails;
        this.category = category;
        this.createdAt = createdAt;
    }

    public UUID getId() { return id; }
    public String getSubject() { return subject; }
    public String getBody() { return body; }
    public String getProvider() { return provider; }
    public List<String> getRecipientEmails() { return recipientEmails; }
    public EmailCategory getCategory() { return category; }
    public LocalDateTime getCreatedAt() { return createdAt; }
}