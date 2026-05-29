package dev.mariinkys.sociospeix.infrastructure.persistence.repository;

import dev.mariinkys.sociospeix.infrastructure.persistence.entity.MemberJpaEntity;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
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

    @Query("""
    SELECT m FROM MemberJpaEntity m
    WHERE (:search IS NULL OR :search = ''
        OR LOWER(m.name)          LIKE LOWER(CONCAT('%', :search, '%'))
        OR LOWER(m.surname)       LIKE LOWER(CONCAT('%', :search, '%'))
        OR LOWER(m.secondSurname) LIKE LOWER(CONCAT('%', :search, '%'))
        OR LOWER(m.email)         LIKE LOWER(CONCAT('%', :search, '%')))
    AND (:interestCount = 0L OR
        (SELECT COUNT(DISTINCT i.id) FROM MemberJpaEntity m2
         JOIN m2.interests i
         WHERE m2.id = m.id AND i.id IN :interestIds) = :interestCount)
    """)
    Page<MemberJpaEntity> findAll(@Param("search") String search,
                                  @Param("interestIds") List<Integer> interestIds,
                                  @Param("interestCount") long interestCount,
                                  Pageable pageable);
}