package dev.mariinkys.cococms.interfaces.dto;

import dev.mariinkys.cococms.domain.model.User;
import java.time.LocalDateTime;
import java.util.UUID;

public class UserResponse {
    private UUID id;
    private String name;
    private String email;
    private LocalDateTime createdAt;

    // Static factory — clean way to build from domain
    public static UserResponse from(User user) {
        UserResponse r = new UserResponse();
        r.id = user.getId();
        r.name = user.getName();
        r.email = user.getEmail();
        r.createdAt = user.getCreatedAt();
        return r;
    }

    public UUID getId() { return id; }
    public String getName() { return name; }
    public String getEmail() { return email; }
    public LocalDateTime getCreatedAt() { return createdAt; }
}
