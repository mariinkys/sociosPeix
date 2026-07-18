package dev.mariinkys.sociospeix.application.port;

public interface PasswordResetUseCase {
    void requestReset(String email);
    void resetPassword(String email, String code, String newPassword);
}
