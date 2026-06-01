package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.application.port.GenderUseCase;
import dev.mariinkys.sociospeix.domain.model.Gender;
import dev.mariinkys.sociospeix.domain.repository.GenderRepository;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import java.util.List;

@Service
public class GenderService implements GenderUseCase {

    private final GenderRepository genderRepository;

    public GenderService(GenderRepository genderRepository) {
        this.genderRepository = genderRepository;
    }

    @Override
    @Transactional(readOnly = true)
    @Cacheable("genders")
    public List<Gender> getAllGenders() {
        return genderRepository.findAll();
    }
}