package dev.mariinkys.sociospeix.application.exception;

import java.util.UUID;

public class EmailNotFoundException extends RuntimeException {
    public EmailNotFoundException(UUID id) {
        super("Email not found with id: " + id);
    }
}
