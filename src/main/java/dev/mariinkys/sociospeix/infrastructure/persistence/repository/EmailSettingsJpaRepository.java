package dev.mariinkys.sociospeix.infrastructure.persistence.repository;

import dev.mariinkys.sociospeix.infrastructure.persistence.entity.EmailSettingsJpaEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface EmailSettingsJpaRepository extends JpaRepository<EmailSettingsJpaEntity, Integer> {
}