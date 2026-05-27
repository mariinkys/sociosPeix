package dev.mariinkys.cococms.interfaces.dto.auth;

import dev.mariinkys.cococms.domain.model.User;
import java.util.UUID;

public record AuthResponse(UUID id, String email, String role) {
    public static AuthResponse from(User user) {
        return new AuthResponse(user.getId(), user.getEmail(), user.getRole().name());
    }
}