package dev.mariinkys.sociospeix.interfaces.dto.user;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;

public record ChangePasswordRequest(
        String currentPassword, // required for regular users, ignored for admins

        @NotBlank(message = "New password is required")
        @Size(min = 8, message = "Password must be at least 8 characters")
        String newPassword
) {}
