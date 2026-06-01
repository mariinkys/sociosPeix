package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.application.port.CountryUseCase;
import dev.mariinkys.sociospeix.domain.model.Country;
import dev.mariinkys.sociospeix.domain.repository.CountryRepository;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import java.util.List;

@Service
public class CountryService implements CountryUseCase {

    private final CountryRepository countryRepository;

    public CountryService(CountryRepository countryRepository) {
        this.countryRepository = countryRepository;
    }

    @Override
    @Transactional(readOnly = true)
    @Cacheable("countries")
    public List<Country> getAllCountries() {
        return countryRepository.findAll();
    }
}