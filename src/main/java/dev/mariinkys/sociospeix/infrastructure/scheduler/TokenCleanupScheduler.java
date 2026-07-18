package dev.mariinkys.sociospeix.infrastructure.scheduler;

import dev.mariinkys.sociospeix.domain.repository.RefreshTokenRepository;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Component;
import org.springframework.transaction.annotation.Transactional;

@Component
public class TokenCleanupScheduler {

    private final RefreshTokenRepository refreshTokenRepository;

    public TokenCleanupScheduler(RefreshTokenRepository refreshTokenRepository) {
        this.refreshTokenRepository = refreshTokenRepository;
    }

    @Scheduled(cron = "0 0 3 * * *") // runs every day at 3am
    @Transactional
    public void cleanUpExpiredTokens() {
        refreshTokenRepository.deleteExpired();
    }
}