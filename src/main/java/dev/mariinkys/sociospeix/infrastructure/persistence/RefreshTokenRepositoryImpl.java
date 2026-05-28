package dev.mariinkys.sociospeix.infrastructure.persistence;

import dev.mariinkys.sociospeix.domain.model.RefreshToken;
import dev.mariinkys.sociospeix.domain.repository.RefreshTokenRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.RefreshTokenJpaEntity;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.RefreshTokenJpaRepository;
import org.springframework.stereotype.Component;

import java.time.LocalDateTime;
import java.util.Optional;

@Component
public class RefreshTokenRepositoryImpl implements RefreshTokenRepository {

    private final RefreshTokenJpaRepository jpaRepository;

    public RefreshTokenRepositoryImpl(RefreshTokenJpaRepository jpaRepository) {
        this.jpaRepository = jpaRepository;
    }

    @Override
    public RefreshToken save(RefreshToken token) {
        var entity = new RefreshTokenJpaEntity(
                token.getId(),
                token.getToken(),
                token.getUserEmail(),
                token.getExpiresAt(),
                token.isRevoked(),
                token.getCreatedAt() != null ? token.getCreatedAt() : LocalDateTime.now()
        );
        var saved = jpaRepository.save(entity);
        return toDomain(saved);
    }

    @Override
    public Optional<RefreshToken> findByToken(String token) {
        return jpaRepository.findByToken(token).map(this::toDomain);
    }

    @Override
    public void revokeAllByUserEmail(String email) {
        jpaRepository.revokeAllByUserEmail(email);
    }

    @Override
    public void deleteExpired() {
        jpaRepository.deleteByExpiresAtBefore(LocalDateTime.now());
    }

    private RefreshToken toDomain(RefreshTokenJpaEntity e) {
        return new RefreshToken(e.getId(), e.getToken(), e.getUserEmail(),
                e.getExpiresAt(), e.isRevoked(), e.getCreatedAt());
    }
}