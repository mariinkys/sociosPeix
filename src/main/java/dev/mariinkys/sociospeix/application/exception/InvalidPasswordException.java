package dev.mariinkys.sociospeix.application.exception;

public class InvalidPasswordException extends RuntimeException {
    public InvalidPasswordException() {
        super("Current password is incorrect");
    }
}
