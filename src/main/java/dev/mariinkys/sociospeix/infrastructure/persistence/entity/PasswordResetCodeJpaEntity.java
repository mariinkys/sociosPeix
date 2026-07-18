package dev.mariinkys.sociospeix.infrastructure.persistence.entity;

import jakarta.persistence.*;

import java.time.LocalDateTime;
import java.util.UUID;

@Entity
@Table(name = "password_reset_codes")
public class PasswordResetCodeJpaEntity {

    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    private UUID id;

    @Column(nullable = false)
    private String email;

    @Column(name = "code_hash", nullable = false)
    private String codeHash;

    @Column(name = "expires_at", nullable = false)
    private LocalDateTime expiresAt;

    @Column(name = "created_at", nullable = false, updatable = false)
    private LocalDateTime createdAt;

    @Column(nullable = false)
    private int attempts = 0;

    @Column(nullable = false)
    private boolean used = false;

    @PrePersist
    protected void onCreate() {
        if (createdAt == null) {
            createdAt = LocalDateTime.now();
        }
    }

    protected PasswordResetCodeJpaEntity() {}

    public PasswordResetCodeJpaEntity(UUID id, String email, String codeHash, LocalDateTime expiresAt,
                                      LocalDateTime createdAt, int attempts, boolean used) {
        this.id = id;
        this.email = email;
        this.codeHash = codeHash;
        this.expiresAt = expiresAt;
        this.createdAt = createdAt;
        this.attempts = attempts;
        this.used = used;
    }

    public UUID getId() { return id; }
    public String getEmail() { return email; }
    public String getCodeHash() { return codeHash; }
    public LocalDateTime getExpiresAt() { return expiresAt; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public int getAttempts() { return attempts; }
    public boolean isUsed() { return used; }
}