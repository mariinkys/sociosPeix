package dev.mariinkys.sociospeix.application.port;

import dev.mariinkys.sociospeix.domain.model.Country;
import java.util.List;

public interface CountryUseCase {
    List<Country> getAllCountries();
}
