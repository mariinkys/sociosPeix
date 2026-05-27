package dev.mariinkys.cococms.domain.repository;

import dev.mariinkys.cococms.domain.model.RefreshToken;
import java.util.Optional;

public interface RefreshTokenRepository {
    RefreshToken save(RefreshToken token);
    Optional<RefreshToken> findByToken(String token);
    void revokeAllByUserEmail(String email); // used on logout
    void deleteExpired(); // for scheduled cleanup
}