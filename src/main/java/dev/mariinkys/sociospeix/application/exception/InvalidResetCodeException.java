package dev.mariinkys.sociospeix.application.exception;

public class InvalidResetCodeException extends RuntimeException {
    public InvalidResetCodeException() {
        super("Invalid, expired, or already-used reset code");
    }
}
