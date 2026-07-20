package dev.mariinkys.sociospeix.interfaces.dto.statistics;

import dev.mariinkys.sociospeix.domain.model.statistics.InterestPopularity;

public record InterestPopularityResponse(String interestName, long memberCount) {
    public static InterestPopularityResponse from(InterestPopularity popularity) {
        return new InterestPopularityResponse(popularity.interestName(), popularity.memberCount());
    }
}
