package dev.mariinkys.sociospeix.interfaces.dto.email;

public record EmailProviderInfoResponse(String name, int dailyLimit) {
    public static EmailProviderInfoResponse from(dev.mariinkys.sociospeix.application.service.EmailProviderInfo info) {
        return new EmailProviderInfoResponse(info.name(), info.dailyLimit());
    }
}