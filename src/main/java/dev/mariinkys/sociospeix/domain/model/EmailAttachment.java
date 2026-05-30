package dev.mariinkys.sociospeix.domain.model;

public record EmailAttachment(String filename, String contentType, byte[] content, String contentId) {
    public EmailAttachment(String filename, String contentType, byte[] content) {
        this(filename, contentType, content, null);
    }

}
