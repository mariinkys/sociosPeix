package dev.mariinkys.sociospeix.infrastructure.persistence.repository;

import dev.mariinkys.sociospeix.infrastructure.persistence.entity.EmailJpaEntity;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

import java.time.LocalDateTime;
import java.util.UUID;

public interface EmailJpaRepository extends JpaRepository<EmailJpaEntity, UUID> {
    @Query("""
        SELECT e FROM EmailJpaEntity e
        WHERE (SELECT m.email FROM MemberJpaEntity m WHERE m.id = :memberId) MEMBER OF e.recipientEmails
    """)
    Page<EmailJpaEntity> findByMemberId(@Param("memberId") UUID memberId, Pageable pageable);

    @Query("""
        SELECT COALESCE(SUM(SIZE(e.recipientEmails)), 0) FROM EmailJpaEntity e
        WHERE e.provider = :provider
          AND e.createdAt >= :startOfDay
    """)
    int countRecipientsToday(@Param("provider") String provider,
                             @Param("startOfDay") LocalDateTime startOfDay);

    @Query(value = """
        SELECT COUNT(*) > 0 FROM emails e
        JOIN email_recipients r ON r.email_id = e.id
        WHERE e.subject = :subject
          AND r.email   = :recipientEmail
          AND e.created_at >= :startOfDay
    """, nativeQuery = true)
    boolean alreadySentTodayTo(@Param("recipientEmail") String recipientEmail,
                               @Param("subject") String subject,
                               @Param("startOfDay") LocalDateTime startOfDay);
}