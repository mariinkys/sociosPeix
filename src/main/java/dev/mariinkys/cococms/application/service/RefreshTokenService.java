package dev.mariinkys.cococms.application.service;

import dev.mariinkys.cococms.application.exception.InvalidRefreshTokenException;
import dev.mariinkys.cococms.domain.model.RefreshToken;
import dev.mariinkys.cococms.domain.repository.RefreshTokenRepository;
import org.springframework.stereotype.Service;

import java.time.LocalDateTime;
import java.util.UUID;

@Service
public class RefreshTokenService {

    private static final long REFRESH_TOKEN_EXPIRY_DAYS = 7;

    private final RefreshTokenRepository refreshTokenRepository;

    public RefreshTokenService(RefreshTokenRepository refreshTokenRepository) {
        this.refreshTokenRepository = refreshTokenRepository;
    }

    public String createRefreshToken(String email) {
        var token = new RefreshToken(
                UUID.randomUUID().toString(),
                email,
                LocalDateTime.now().plusDays(REFRESH_TOKEN_EXPIRY_DAYS)
        );
        return refreshTokenRepository.save(token).getToken();
    }

    // Validates the token, revokes it, and returns the owner's email.
    // The caller is responsible for issuing a new access token + refresh token.
    public String validateAndRotate(String tokenValue) {
        RefreshToken token = refreshTokenRepository.findByToken(tokenValue)
                .orElseThrow(() -> new InvalidRefreshTokenException("Refresh token not found"));

        if (!token.isValid()) {
            // If already revoked, this could be a token reuse attack, revoke everything
            if (token.isRevoked()) {
                refreshTokenRepository.revokeAllByUserEmail(token.getUserEmail());
                throw new InvalidRefreshTokenException("Token reuse detected — all sessions revoked");
            }
            throw new InvalidRefreshTokenException("Refresh token has expired");
        }

        // Rotate: save revoked version, caller will create a new one
        refreshTokenRepository.save(token.revoke());
        return token.getUserEmail();
    }

    public void revokeAllForUser(String email) {
        refreshTokenRepository.revokeAllByUserEmail(email);
    }
}