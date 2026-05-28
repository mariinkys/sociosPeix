package dev.mariinkys.sociospeix.infrastructure.persistence.repository;

import dev.mariinkys.sociospeix.infrastructure.persistence.entity.GenderJpaEntity;
import org.springframework.data.jpa.repository.JpaRepository;

public interface GenderJpaRepository extends JpaRepository<GenderJpaEntity, Integer> {}