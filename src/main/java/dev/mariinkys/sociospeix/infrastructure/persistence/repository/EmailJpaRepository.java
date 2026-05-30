package dev.mariinkys.sociospeix.infrastructure.persistence.repository;

import dev.mariinkys.sociospeix.infrastructure.persistence.entity.EmailJpaEntity;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

import java.util.UUID;

public interface EmailJpaRepository extends JpaRepository<EmailJpaEntity, UUID> {
    @Query("""
        SELECT e FROM EmailJpaEntity e
        WHERE (SELECT m.email FROM MemberJpaEntity m WHERE m.id = :memberId) MEMBER OF e.recipientEmails
    """)
    Page<EmailJpaEntity> findByMemberId(@Param("memberId") UUID memberId, Pageable pageable);
}