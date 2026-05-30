package dev.mariinkys.sociospeix.application.service;

public record EmailProviderStatus(String provider, int dailyLimit, int sentToday, int remaining) {}
