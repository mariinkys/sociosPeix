package dev.mariinkys.cococms.infrastructure.scheduler;

import dev.mariinkys.cococms.domain.repository.RefreshTokenRepository;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Component;

@Component
public class TokenCleanupScheduler {

    private final RefreshTokenRepository refreshTokenRepository;

    public TokenCleanupScheduler(RefreshTokenRepository refreshTokenRepository) {
        this.refreshTokenRepository = refreshTokenRepository;
    }

    @Scheduled(cron = "0 0 3 * * *") // runs every day at 3am
    public void cleanUpExpiredTokens() {
        refreshTokenRepository.deleteExpired();
    }
}