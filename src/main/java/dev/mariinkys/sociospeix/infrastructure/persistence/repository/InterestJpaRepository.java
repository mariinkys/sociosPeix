package dev.mariinkys.sociospeix.infrastructure.persistence.repository;

import dev.mariinkys.sociospeix.infrastructure.persistence.entity.InterestJpaEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import java.util.List;
import java.util.UUID;

public interface InterestJpaRepository extends JpaRepository<InterestJpaEntity, Integer> {

    @Query("""
        SELECT i FROM InterestJpaEntity i
        JOIN MemberJpaEntity m ON i MEMBER OF m.interests
        WHERE m.id = :memberId
    """)
    List<InterestJpaEntity> findByMemberId(@Param("memberId") UUID memberId);
}