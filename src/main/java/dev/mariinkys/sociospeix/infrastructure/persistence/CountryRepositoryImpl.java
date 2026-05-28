package dev.mariinkys.sociospeix.infrastructure.persistence;

import dev.mariinkys.sociospeix.domain.model.Country;
import dev.mariinkys.sociospeix.domain.repository.CountryRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.CountryJpaRepository;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.Optional;

@Component
public class CountryRepositoryImpl implements CountryRepository {

    private final CountryJpaRepository jpaRepository;

    public CountryRepositoryImpl(CountryJpaRepository jpaRepository) {
        this.jpaRepository = jpaRepository;
    }

    @Override
    public List<Country> findAll() {
        return jpaRepository.findAll()
                .stream()
                .map(e -> new Country(e.getId(), e.getName()))
                .toList();
    }

    @Override
    public Optional<Country> findById(Integer id) {
        return jpaRepository.findById(id)
                .map(e -> new Country(e.getId(), e.getName()));
    }
}