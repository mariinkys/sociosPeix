package dev.mariinkys.sociospeix.domain.repository;

import dev.mariinkys.sociospeix.domain.model.PasswordResetCode;

import java.util.Optional;

public interface PasswordResetCodeRepository {
    PasswordResetCode save(PasswordResetCode code);
    Optional<PasswordResetCode> findActiveByEmail(String email); // latest non-used code for the email
    void invalidateAllForEmail(String email);
    void deleteExpired();
}