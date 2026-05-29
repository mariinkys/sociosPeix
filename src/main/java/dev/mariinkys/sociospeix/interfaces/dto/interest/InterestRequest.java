package dev.mariinkys.sociospeix.interfaces.dto.interest;

import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;

public record InterestRequest(
        @NotBlank(message = "Name is required")
        @Size(max = 100, message = "Name must not exceed 100 characters")
        String name,

        String description
) {}
