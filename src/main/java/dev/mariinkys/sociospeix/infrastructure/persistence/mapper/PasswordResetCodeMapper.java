package dev.mariinkys.sociospeix.infrastructure.persistence.mapper;

import dev.mariinkys.sociospeix.domain.model.PasswordResetCode;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.PasswordResetCodeJpaEntity;
import org.springframework.stereotype.Component;

@Component
public class PasswordResetCodeMapper {

    public PasswordResetCode toDomain(PasswordResetCodeJpaEntity entity) {
        return new PasswordResetCode(
                entity.getId(),
                entity.getEmail(),
                entity.getCodeHash(),
                entity.getExpiresAt(),
                entity.getCreatedAt(),
                entity.getAttempts(),
                entity.isUsed()
        );
    }

    public PasswordResetCodeJpaEntity toEntity(PasswordResetCode code) {
        return new PasswordResetCodeJpaEntity(
                code.getId(),
                code.getEmail(),
                code.getCodeHash(),
                code.getExpiresAt(),
                code.getCreatedAt(),
                code.getAttempts(),
                code.isUsed()
        );
    }
}