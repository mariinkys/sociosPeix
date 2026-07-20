package dev.mariinkys.sociospeix.interfaces.dto.email;

import dev.mariinkys.sociospeix.application.service.MultiEmailCheckResult;

public record MultiEmailCheckResponse(
        String provider,
        int dailyLimit,
        int sentToday,
        int remaining,
        int totalRecipients,
        boolean exceedsLimit
) {
    public static MultiEmailCheckResponse from(MultiEmailCheckResult result) {
        return new MultiEmailCheckResponse(
                result.provider(),
                result.dailyLimit(),
                result.sentToday(),
                result.remaining(),
                result.totalRecipients(),
                result.exceedsLimit()
        );
    }
}