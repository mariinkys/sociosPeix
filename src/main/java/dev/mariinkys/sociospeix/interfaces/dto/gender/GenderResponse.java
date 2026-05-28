package dev.mariinkys.sociospeix.interfaces.dto.gender;

import dev.mariinkys.sociospeix.domain.model.Gender;

public record GenderResponse(Integer id, String name) {
    public static GenderResponse from(Gender gender) {
        return new GenderResponse(gender.getId(), gender.getName());
    }
}