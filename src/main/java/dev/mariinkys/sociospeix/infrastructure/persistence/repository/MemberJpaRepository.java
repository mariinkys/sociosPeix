package dev.mariinkys.sociospeix.infrastructure.persistence.repository;

import dev.mariinkys.sociospeix.infrastructure.persistence.entity.MemberJpaEntity;
import org.springframework.data.domain.Sort;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import java.util.List;
import java.util.Optional;
import java.util.UUID;

public interface MemberJpaRepository extends JpaRepository<MemberJpaEntity, UUID> {

    // Single member with all associations loaded
    @Query("""
        SELECT m FROM MemberJpaEntity m
        LEFT JOIN FETCH m.interests
        LEFT JOIN FETCH m.gender
        LEFT JOIN FETCH m.country
        WHERE m.id = :id
        """)
    Optional<MemberJpaEntity> findByIdWithDetails(@Param("id") UUID id);

    // Step 1 of pagination we get matching IDs only
    @Query("""
    SELECT m.id FROM MemberJpaEntity m
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
    List<UUID> findPageIds(@Param("search") String search,
                           @Param("interestIds") List<Integer> interestIds,
                           @Param("interestCount") long interestCount);

    // Step 2 of pagination we fetch full entities with JOIN FETCH for the given IDs
    @Query("""
    SELECT DISTINCT m FROM MemberJpaEntity m
    LEFT JOIN FETCH m.interests
    LEFT JOIN FETCH m.gender
    LEFT JOIN FETCH m.country
    WHERE m.id IN :ids
    """)
    List<MemberJpaEntity> findAllByIdWithDetails(@Param("ids") List<UUID> ids, Sort sort);

    // Count query for pagination total
    @Query("""
        SELECT COUNT(m) FROM MemberJpaEntity m
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
    long countWithFilters(@Param("search") String search,
                          @Param("interestIds") List<Integer> interestIds,
                          @Param("interestCount") long interestCount);

    @Query("""
        SELECT DISTINCT m FROM MemberJpaEntity m
        LEFT JOIN FETCH m.interests
        LEFT JOIN FETCH m.gender
        LEFT JOIN FETCH m.country
        """)
    List<MemberJpaEntity> findAllMembers();

    // Members with any of the given interests
    @Query("""
        SELECT DISTINCT m FROM MemberJpaEntity m
        LEFT JOIN FETCH m.interests
        LEFT JOIN FETCH m.gender
        LEFT JOIN FETCH m.country
        WHERE EXISTS (
            SELECT 1 FROM MemberJpaEntity m2
            JOIN m2.interests i
            WHERE m2.id = m.id AND i.id IN :interestIds
        )
        """)
    List<MemberJpaEntity> findAllByAnyInterestId(@Param("interestIds") List<Integer> interestIds);

    @Query("""
        SELECT DISTINCT m FROM MemberJpaEntity m
        LEFT JOIN FETCH m.interests
        LEFT JOIN FETCH m.gender
        LEFT JOIN FETCH m.country
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
    List<MemberJpaEntity> findAllForExport(@Param("search") String search,
                                           @Param("interestIds") List<Integer> interestIds,
                                           @Param("interestCount") long interestCount);

    @Query(value = """
        SELECT * FROM members
        WHERE birthdate IS NOT NULL
          AND EXTRACT(DAY FROM birthdate)   = :day
          AND EXTRACT(MONTH FROM birthdate) = :month
        ORDER BY name
        """, nativeQuery = true)
    List<MemberJpaEntity> findByBirthday(@Param("day") int day, @Param("month") int month);
}