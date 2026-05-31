package dev.mariinkys.sociospeix.interfaces.dto.user;

import dev.mariinkys.sociospeix.domain.model.Role;
import jakarta.validation.constraints.NotNull;

public record UpdateRoleRequest(
        @NotNull(message = "Role is required")
        Role role
) {}
