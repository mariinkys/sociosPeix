package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.domain.repository.UserRepository;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

import java.time.LocalDateTime;

@Service
public class LoginAttemptService {

    @Value("${app.security.max-login-attempts}")
    private int maxAttempts;

    @Value("${app.security.lockout-duration-minutes}")
    private int lockoutMinutes;

    private final UserRepository userRepository;

    public LoginAttemptService(UserRepository userRepository) {
        this.userRepository = userRepository;
    }

    public void onSuccess(String email) {
        userRepository.findByEmail(email).ifPresent(user -> userRepository.save(user.resetFailedAttempts()));
    }

    public void onFailure(String email) {
        userRepository.findByEmail(email).ifPresent(user -> {
            var updated = user.incrementFailedAttempts();
            if (updated.getFailedLoginAttempts() >= maxAttempts) {
                updated = updated.lockUntil(LocalDateTime.now().plusMinutes(lockoutMinutes));
            }
            userRepository.save(updated);
        });
    }
}