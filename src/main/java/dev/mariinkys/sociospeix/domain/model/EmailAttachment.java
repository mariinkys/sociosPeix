package dev.mariinkys.sociospeix.domain.model;

public record EmailAttachment(String filename, String contentType, byte[] content) {}
