package dev.mariinkys.cococms.domain.model;

import java.time.LocalDateTime;
import java.util.UUID;

public class User {

    private UUID id;
    private final String name;
    private final String email;
    private final String password;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;

    // Constructor for creating a NEW user
    public User(String name, String email, String hashedPassword) {
        this.name = name;
        this.email = email;
        this.password = hashedPassword;
    }

    // Constructor for reconstructing from DB
    public User(UUID id, String name, String email, String password,
                LocalDateTime createdAt, LocalDateTime updatedAt) {
        this.id = id;
        this.name = name;
        this.email = email;
        this.password = password;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    // Getters only — immutable where possible
    public UUID getId() { return id; }
    public String getName() { return name; }
    public String getEmail() { return email; }
    public String getPassword() { return password; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public LocalDateTime getUpdatedAt() { return updatedAt; }

    // Update method — domain logic lives here, not in the service
    public User withUpdatedDetails(String name, String email) {
        return new User(this.id, name, email, this.password, this.createdAt, LocalDateTime.now());
    }
}
