package dev.mariinkys.sociospeix.infrastructure.persistence.repository;

import dev.mariinkys.sociospeix.infrastructure.persistence.entity.MemberJpaEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

import java.util.List;
import java.util.UUID;

public interface MemberJpaRepository extends JpaRepository<MemberJpaEntity, UUID> {

    @Query(value = """
    SELECT * FROM members
    WHERE birthdate IS NOT NULL
      AND EXTRACT(DAY FROM birthdate)   = :day
      AND EXTRACT(MONTH FROM birthdate) = :month
    ORDER BY name ASC
    """, nativeQuery = true)
    List<MemberJpaEntity> findByBirthday(@Param("day") int day, @Param("month") int month);
}