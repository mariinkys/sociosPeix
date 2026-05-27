package dev.mariinkys.cococms.domain.model;

import java.time.LocalDateTime;
import java.util.UUID;

public class User {

    private UUID id;
    private final String name;
    private final String email;
    private final String password;
    private final Role role;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;

    // Constructor for creating a new user
    public User(String name, String email, String hashedPassword) {
        this.name = name;
        this.email = email;
        this.role = Role.USER;
        this.password = hashedPassword;
    }

    // Constructor for reconstructing from DB
    public User(UUID id, String name, String email, String password, Role role,
                LocalDateTime createdAt, LocalDateTime updatedAt) {
        this.id = id;
        this.name = name;
        this.email = email;
        this.password = password;
        this.role = role;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    public UUID getId() { return id; }
    public String getName() { return name; }
    public String getEmail() { return email; }
    public String getPassword() { return password; }
    public Role getRole() { return role; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public LocalDateTime getUpdatedAt() { return updatedAt; }

    // domain logic lives here, not in the service (i have to remember clean arch :c)
    public User withUpdatedDetails(String name, String email) {
        return new User(this.id, name, email, this.password, this.role, this.createdAt, LocalDateTime.now());
    }
}
