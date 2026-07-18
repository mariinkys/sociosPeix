package dev.mariinkys.sociospeix.domain.repository;

import java.util.Optional;

public interface EmailSettingsRepository {
    Optional<String> getActiveProviderName();
    void setActiveProviderName(String providerName);
}