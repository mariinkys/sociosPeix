package dev.mariinkys.sociospeix.interfaces.dto.auth;

import dev.mariinkys.sociospeix.domain.model.User;
import java.util.UUID;

public record AuthResponse(UUID id, String email, String role) {
    public static AuthResponse from(User user) {
        return new AuthResponse(user.getId(), user.getEmail(), user.getRole().name());
    }
}