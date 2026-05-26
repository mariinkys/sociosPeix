package dev.mariinkys.cococms.domain.model;

import java.time.LocalDateTime;
import java.util.UUID;

public class User {

    private UUID id;
    private String name;
    private String email;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;

    // Constructor for creating a NEW user
    public User(String name, String email) {
        this.name = name;
        this.email = email;
    }

    // Constructor for reconstructing from DB
    public User(UUID id, String name, String email,
                LocalDateTime createdAt, LocalDateTime updatedAt) {
        this.id = id;
        this.name = name;
        this.email = email;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    // Getters only — immutable where possible
    public UUID getId() { return id; }
    public String getName() { return name; }
    public String getEmail() { return email; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public LocalDateTime getUpdatedAt() { return updatedAt; }

    // Update method — domain logic lives here, not in the service
    public User withUpdatedDetails(String name, String email) {
        return new User(this.id, name, email, this.createdAt, LocalDateTime.now());
    }
}
