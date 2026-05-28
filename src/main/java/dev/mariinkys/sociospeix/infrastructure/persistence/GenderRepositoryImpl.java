package dev.mariinkys.sociospeix.infrastructure.persistence;

import dev.mariinkys.sociospeix.domain.model.Gender;
import dev.mariinkys.sociospeix.domain.repository.GenderRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.GenderJpaRepository;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.Optional;

@Component
public class GenderRepositoryImpl implements GenderRepository {

    private final GenderJpaRepository jpaRepository;

    public GenderRepositoryImpl(GenderJpaRepository jpaRepository) {
        this.jpaRepository = jpaRepository;
    }

    @Override
    public List<Gender> findAll() {
        return jpaRepository.findAll()
                .stream()
                .map(e -> new Gender(e.getId(), e.getName()))
                .toList();
    }

    @Override
    public Optional<Gender> findById(Integer id) {
        return jpaRepository.findById(id)
                .map(e -> new Gender(e.getId(), e.getName()));
    }
}