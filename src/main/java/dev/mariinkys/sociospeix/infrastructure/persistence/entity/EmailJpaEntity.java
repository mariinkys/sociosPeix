package dev.mariinkys.sociospeix.infrastructure.persistence.entity;

import dev.mariinkys.sociospeix.domain.model.EmailCategory;
import jakarta.persistence.*;

import java.time.LocalDateTime;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

@Entity
@Table(name = "emails")
public class EmailJpaEntity {

    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    private UUID id;

    @Column(nullable = false, length = 255)
    private String subject;

    @Column(nullable = false, columnDefinition = "TEXT")
    private String body;

    @Column(nullable = false, length = 50)
    private String provider;

    // JPA manages the email_recipients table automatically
    @ElementCollection(fetch = FetchType.LAZY)
    @CollectionTable(name = "email_recipients",
            joinColumns = @JoinColumn(name = "email_id"))
    @Column(name = "email", nullable = false, length = 255)
    private List<String> recipientEmails = new ArrayList<>();

    @Enumerated(EnumType.STRING)
    @Column(nullable = false, length = 20)
    private EmailCategory category;

    @Column(name = "created_at", nullable = false, updatable = false)
    private LocalDateTime createdAt;

    @PrePersist
    protected void onCreate() { createdAt = LocalDateTime.now(); }

    protected EmailJpaEntity() {}

    // For creating
    public EmailJpaEntity(String subject, String body, String provider,
                          List<String> recipientEmails, EmailCategory category) {
        this.subject = subject;
        this.body = body;
        this.provider = provider;
        this.recipientEmails = new ArrayList<>(recipientEmails);
        this.category = category;
    }

    // For reconstructing from DB
    public EmailJpaEntity(UUID id, String subject, String body, String provider,
                          List<String> recipientEmails, EmailCategory category, LocalDateTime createdAt) {
        this.id = id;
        this.subject = subject;
        this.body = body;
        this.provider = provider;
        this.recipientEmails = new ArrayList<>(recipientEmails);
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