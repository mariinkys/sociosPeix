package dev.mariinkys.sociospeix.application.exception;

import java.util.UUID;

public class MemberNotFoundException extends RuntimeException {
    public MemberNotFoundException(UUID id) {
        super("Member not found with id: " + id);
    }
}
