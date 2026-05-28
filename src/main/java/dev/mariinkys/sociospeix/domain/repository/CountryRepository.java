package dev.mariinkys.sociospeix.domain.repository;

import dev.mariinkys.sociospeix.domain.model.Country;
import java.util.List;
import java.util.Optional;

public interface CountryRepository {
    List<Country> findAll();
    Optional<Country> findById(Integer id);
}
