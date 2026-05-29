package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.application.exception.InterestNotFoundException;
import dev.mariinkys.sociospeix.application.port.InterestUseCase;
import dev.mariinkys.sociospeix.domain.model.Interest;
import dev.mariinkys.sociospeix.domain.repository.InterestRepository;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import java.util.List;

import static dev.mariinkys.sociospeix.application.utils.CommonUtils.nullToEmpty;

@Service
public class InterestService implements InterestUseCase {

    private final InterestRepository interestRepository;

    public InterestService(InterestRepository interestRepository) {
        this.interestRepository = interestRepository;
    }

    @Override
    @Transactional
    public Interest createInterest(String name, String description) {
        return interestRepository.save(new Interest(null, name, nullToEmpty(description)));
    }

    @Override
    @Transactional(readOnly = true)
    public Interest getInterestById(Integer id) {
        return interestRepository.findById(id)
                .orElseThrow(() -> new InterestNotFoundException(id));
    }

    @Override
    @Transactional(readOnly = true)
    public List<Interest> getAllInterests() {
        return interestRepository.findAll();
    }

    @Override
    @Transactional
    public Interest updateInterest(Integer id, String name, String description) {
        getInterestById(id); // throws if not found
        return interestRepository.save(new Interest(id, name, nullToEmpty(description)));
    }

    @Override
    @Transactional
    public void deleteInterest(Integer id) {
        getInterestById(id);
        interestRepository.deleteById(id);
    }
}