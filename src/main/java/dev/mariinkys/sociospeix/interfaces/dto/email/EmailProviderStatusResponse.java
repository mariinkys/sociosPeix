package dev.mariinkys.sociospeix.interfaces.dto.email;

import dev.mariinkys.sociospeix.application.service.EmailProviderStatus;

public record EmailProviderStatusResponse(
        String provider,
        int dailyLimit,
        int sentToday,
        int remaining
) {
    public static EmailProviderStatusResponse from(EmailProviderStatus status) {
        return new EmailProviderStatusResponse(
                status.provider(),
                status.dailyLimit(),
                status.sentToday(),
                status.remaining()
        );
    }
}
