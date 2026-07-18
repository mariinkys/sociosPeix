package dev.mariinkys.sociospeix.infrastructure.scheduler;

import dev.mariinkys.sociospeix.domain.repository.PasswordResetCodeRepository;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Component;
import org.springframework.transaction.annotation.Transactional;

@Component
public class PasswordResetCodeCleanupScheduler {

    private final PasswordResetCodeRepository passwordResetCodeRepository;

    public PasswordResetCodeCleanupScheduler(PasswordResetCodeRepository passwordResetCodeRepository) {
        this.passwordResetCodeRepository = passwordResetCodeRepository;
    }

    @Scheduled(cron = "0 30 3 * * *") // runs every day at 3:30am
    @Transactional
    public void cleanUpExpiredResetCodes() {
        passwordResetCodeRepository.deleteExpired();
    }
}