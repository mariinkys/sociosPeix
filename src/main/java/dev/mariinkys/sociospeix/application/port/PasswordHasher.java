package dev.mariinkys.sociospeix.application.port;

public interface PasswordHasher {
    String hash(String rawPassword);
}
