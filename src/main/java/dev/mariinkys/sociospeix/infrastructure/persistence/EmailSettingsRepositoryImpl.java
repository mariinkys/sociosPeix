package dev.mariinkys.sociospeix.infrastructure.persistence;

import dev.mariinkys.sociospeix.domain.repository.EmailSettingsRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.EmailSettingsJpaEntity;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.EmailSettingsJpaRepository;
import org.springframework.stereotype.Component;

import java.util.Optional;

@Component
public class EmailSettingsRepositoryImpl implements EmailSettingsRepository {

    private static final int SETTINGS_ROW_ID = 1;

    private final EmailSettingsJpaRepository jpaRepository;

    public EmailSettingsRepositoryImpl(EmailSettingsJpaRepository jpaRepository) {
        this.jpaRepository = jpaRepository;
    }

    @Override
    public Optional<String> getActiveProviderName() {
        return jpaRepository.findById(SETTINGS_ROW_ID).map(EmailSettingsJpaEntity::getActiveProvider);
    }

    @Override
    public void setActiveProviderName(String providerName) {
        var entity = jpaRepository.findById(SETTINGS_ROW_ID)
                .orElse(new EmailSettingsJpaEntity(SETTINGS_ROW_ID, providerName));
        entity.setActiveProvider(providerName);
        jpaRepository.save(entity);
    }
}