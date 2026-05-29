package dev.mariinkys.sociospeix.infrastructure.persistence;

import dev.mariinkys.sociospeix.domain.model.Interest;
import dev.mariinkys.sociospeix.domain.repository.InterestRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.InterestJpaEntity;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.InterestJpaRepository;
import org.springframework.stereotype.Component;
import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Component
public class InterestRepositoryImpl implements InterestRepository {

    private final InterestJpaRepository jpaRepository;

    public InterestRepositoryImpl(InterestJpaRepository jpaRepository) {
        this.jpaRepository = jpaRepository;
    }

    @Override
    public Interest save(Interest interest) {
        return toDomain(jpaRepository.save(toEntity(interest)));
    }

    @Override
    public Optional<Interest> findById(Integer id) {
        return jpaRepository.findById(id).map(this::toDomain);
    }

    @Override
    public List<Interest> findAll() {
        return jpaRepository.findAll().stream().map(this::toDomain).toList();
    }

    @Override
    public List<Interest> findByMemberId(UUID memberId) {
        return jpaRepository.findByMemberId(memberId).stream().map(this::toDomain).toList();
    }

    @Override
    public void deleteById(Integer id) {
        jpaRepository.deleteById(id);
    }

    private Interest toDomain(InterestJpaEntity e) {
        return new Interest(e.getId(), e.getName(), e.getDescription());
    }

    private InterestJpaEntity toEntity(Interest i) {
        return new InterestJpaEntity(i.getId(), i.getName(), i.getDescription());
    }
}