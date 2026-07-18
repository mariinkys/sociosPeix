package dev.mariinkys.sociospeix.infrastructure.persistence.repository;

import dev.mariinkys.sociospeix.infrastructure.persistence.entity.PasswordResetCodeJpaEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

import java.time.LocalDateTime;
import java.util.Optional;
import java.util.UUID;

public interface PasswordResetCodeJpaRepository extends JpaRepository<PasswordResetCodeJpaEntity, UUID> {
    Optional<PasswordResetCodeJpaEntity> findFirstByEmailOrderByCreatedAtDesc(String email);
    void deleteByEmail(String email);

    @Modifying
    @Query("DELETE FROM PasswordResetCodeJpaEntity c WHERE c.expiresAt < :cutoff")
    void deleteByExpiresAtBefore(@Param("cutoff") LocalDateTime cutoff);
}