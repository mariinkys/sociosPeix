package dev.mariinkys.sociospeix.application.service;

import java.util.UUID;

public record RequesterContext(UUID id, String email, boolean isAdmin) {}