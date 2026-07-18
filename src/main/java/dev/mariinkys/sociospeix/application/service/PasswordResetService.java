package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.application.exception.InvalidResetCodeException;
import dev.mariinkys.sociospeix.application.port.EmailUseCase;
import dev.mariinkys.sociospeix.application.port.PasswordHasher;
import dev.mariinkys.sociospeix.application.port.PasswordResetUseCase;
import dev.mariinkys.sociospeix.domain.model.PasswordResetCode;
import dev.mariinkys.sociospeix.domain.model.User;
import dev.mariinkys.sociospeix.domain.repository.PasswordResetCodeRepository;
import dev.mariinkys.sociospeix.domain.repository.UserRepository;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.security.SecureRandom;
import java.nio.charset.StandardCharsets;
import java.time.LocalDateTime;
import java.util.Base64;

@Service
public class PasswordResetService implements PasswordResetUseCase {

    private static final Logger log = LoggerFactory.getLogger(PasswordResetService.class);
    private static final int CODE_TTL_MINUTES = 15;
    private static final int RESEND_COOLDOWN_MINUTES = 5;
    private static final SecureRandom RANDOM = new SecureRandom();

    private final UserRepository userRepository;
    private final PasswordResetCodeRepository codeRepository;
    private final PasswordHasher passwordHasher;
    private final EmailUseCase emailUseCase;
    private final RefreshTokenService refreshTokenService;

    public PasswordResetService(UserRepository userRepository,
                                PasswordResetCodeRepository codeRepository,
                                PasswordHasher passwordHasher,
                                EmailUseCase emailUseCase,
                                RefreshTokenService refreshTokenService) {
        this.userRepository = userRepository;
        this.codeRepository = codeRepository;
        this.passwordHasher = passwordHasher;
        this.emailUseCase = emailUseCase;
        this.refreshTokenService = refreshTokenService;
    }

    @Override
    @Transactional
    public void requestReset(String email) {
        var userOpt = userRepository.findByEmail(email);
        if (userOpt.isEmpty()) {
            log.info("Password reset requested for an email with no matching account");
            return; // same outward behavior whether the account exists
        }

        var existingCode = codeRepository.findActiveByEmail(email);
        if (existingCode.isPresent() && isWithinCooldown(existingCode.get())) {
            log.info("Password reset requested too soon after a previous request; skipping");
            return; // stay silent - identical response to the "no account" case above
        }

        String code = generateCode();
        codeRepository.invalidateAllForEmail(email);
        codeRepository.save(new PasswordResetCode(
                email, hash(code), LocalDateTime.now().plusMinutes(CODE_TTL_MINUTES)));

        try {
            var _ = emailUseCase.sendTransactional(email, "Your password reset code", buildEmailHtml(code));
        } catch (Exception e) {
            // Log but don't leak send failures to the caller - same response either way
            log.error("Failed to send password reset email", e);
        }
    }

    @Override
    @Transactional
    public void resetPassword(String email, String code, String newPassword) {
        PasswordResetCode resetCode = codeRepository.findActiveByEmail(email)
                .orElseThrow(InvalidResetCodeException::new);

        if (!resetCode.isUsable()) {
            throw new InvalidResetCodeException();
        }

        if (!hash(code).equals(resetCode.getCodeHash())) {
            resetCode.registerFailedAttempt();
            codeRepository.save(resetCode);
            throw new InvalidResetCodeException();
        }

        User user = userRepository.findByEmail(email)
                .orElseThrow(InvalidResetCodeException::new);

        userRepository.save(user.withPassword(passwordHasher.hash(newPassword)));

        resetCode.markUsed();
        codeRepository.save(resetCode);

        refreshTokenService.revokeAllForUser(email);
    }

    private boolean isWithinCooldown(PasswordResetCode code) {
        return code.getCreatedAt().plusMinutes(RESEND_COOLDOWN_MINUTES).isAfter(LocalDateTime.now());
    }

    private String generateCode() {
        int value = RANDOM.nextInt(1_000_000); // 0..999999
        return String.format("%06d", value);
    }

    private String hash(String code) {
        try {
            var digest = MessageDigest.getInstance("SHA-256");
            return Base64.getEncoder().encodeToString(digest.digest(code.getBytes(StandardCharsets.UTF_8)));
        } catch (NoSuchAlgorithmException e) {
            throw new IllegalStateException(e);
        }
    }

    private String buildEmailHtml(String code) {
        return """
                <p>Use the code below to reset your password. It expires in %d minutes.</p>
                <h2>%s</h2>
                <p>If you didn't request this, you can safely ignore this email.</p>
                """.formatted(CODE_TTL_MINUTES, code);
    }
}