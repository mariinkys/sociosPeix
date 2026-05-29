package dev.mariinkys.sociospeix.application.exception;

public class InterestNotFoundException extends RuntimeException {
    public InterestNotFoundException(Integer id) {
        super("Interest not found with id: " + id);
    }
}
