package dev.mariinkys.sociospeix.infrastructure.persistence;

import dev.mariinkys.sociospeix.domain.model.PasswordResetCode;
import dev.mariinkys.sociospeix.domain.repository.PasswordResetCodeRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.mapper.PasswordResetCodeMapper;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.PasswordResetCodeJpaRepository;
import org.springframework.stereotype.Component;

import java.time.LocalDateTime;
import java.util.Optional;

@Component
public class PasswordResetCodeRepositoryImpl implements PasswordResetCodeRepository {

    private final PasswordResetCodeJpaRepository jpaRepository;
    private final PasswordResetCodeMapper mapper;

    public PasswordResetCodeRepositoryImpl(PasswordResetCodeJpaRepository jpaRepository,
                                           PasswordResetCodeMapper mapper) {
        this.jpaRepository = jpaRepository;
        this.mapper = mapper;
    }

    @Override
    public PasswordResetCode save(PasswordResetCode code) {
        var entity = mapper.toEntity(code);
        var saved = jpaRepository.save(entity);
        return mapper.toDomain(saved);
    }

    @Override
    public Optional<PasswordResetCode> findActiveByEmail(String email) {
        return jpaRepository.findFirstByEmailOrderByCreatedAtDesc(email).map(mapper::toDomain);
    }

    @Override
    public void invalidateAllForEmail(String email) {
        jpaRepository.deleteByEmail(email);
    }

    @Override
    public void deleteExpired() {
        jpaRepository.deleteByExpiresAtBefore(LocalDateTime.now().minusDays(1));
    }
}