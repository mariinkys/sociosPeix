package dev.mariinkys.cococms.infrastructure.persistence.repository;

import dev.mariinkys.cococms.infrastructure.persistence.entity.UserJpaEntity;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.Optional;
import java.util.UUID;

// Spring generates all the CRUD SQL automatically from this interface
public interface UserJpaRepository extends JpaRepository<UserJpaEntity, UUID> {
    boolean existsByEmail(String email);
    Optional<UserJpaEntity> findByEmail(String email);
}