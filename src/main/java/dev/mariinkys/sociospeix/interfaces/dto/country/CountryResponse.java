package dev.mariinkys.sociospeix.interfaces.dto.country;

import dev.mariinkys.sociospeix.domain.model.Country;

public record CountryResponse(Integer id, String name) {
    public static CountryResponse from(Country country) {
        return new CountryResponse(country.getId(), country.getName());
    }
}
