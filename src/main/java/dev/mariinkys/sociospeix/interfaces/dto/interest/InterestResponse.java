package dev.mariinkys.sociospeix.interfaces.dto.interest;

import dev.mariinkys.sociospeix.domain.model.Interest;

public record InterestResponse(Integer id, String name, String description) {
    public static InterestResponse from(Interest interest) {
        return new InterestResponse(interest.getId(), interest.getName(), interest.getDescription());
    }
}
