package dev.mariinkys.sociospeix.application.service;

public record MultiEmailCheckResult(
        String provider,
        int dailyLimit,
        int sentToday,
        int remaining,
        int totalRecipients,
        boolean exceedsLimit
) {}