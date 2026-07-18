package dev.mariinkys.sociospeix.domain.model;

import java.time.LocalDateTime;
import java.util.UUID;

public class PasswordResetCode {

    private static final int MAX_ATTEMPTS = 5;

    private final UUID id;
    private final String email;
    private final String codeHash;
    private final LocalDateTime expiresAt;
    private final LocalDateTime createdAt;
    private int attempts;
    private boolean used;

    public PasswordResetCode(String email, String codeHash, LocalDateTime expiresAt) {
        this(null, email, codeHash, expiresAt, LocalDateTime.now(), 0, false);
    }

    public PasswordResetCode(UUID id, String email, String codeHash, LocalDateTime expiresAt,
                             LocalDateTime createdAt, int attempts, boolean used) {
        this.id = id;
        this.email = email;
        this.codeHash = codeHash;
        this.expiresAt = expiresAt;
        this.createdAt = createdAt;
        this.attempts = attempts;
        this.used = used;
    }

    public boolean isExpired() {
        return LocalDateTime.now().isAfter(expiresAt);
    }

    public boolean isLocked() {
        return attempts >= MAX_ATTEMPTS;
    }

    public boolean isUsable() {
        return !used && !isExpired() && !isLocked();
    }

    public void registerFailedAttempt() {
        this.attempts++;
    }

    public void markUsed() {
        this.used = true;
    }

    public UUID getId() { return id; }
    public String getEmail() { return email; }
    public String getCodeHash() { return codeHash; }
    public LocalDateTime getExpiresAt() { return expiresAt; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public int getAttempts() { return attempts; }
    public boolean isUsed() { return used; }
}