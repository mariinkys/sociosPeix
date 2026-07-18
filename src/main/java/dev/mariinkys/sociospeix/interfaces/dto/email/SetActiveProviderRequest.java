package dev.mariinkys.sociospeix.interfaces.dto.email;

import jakarta.validation.constraints.NotBlank;

public record SetActiveProviderRequest(@NotBlank String provider) {}