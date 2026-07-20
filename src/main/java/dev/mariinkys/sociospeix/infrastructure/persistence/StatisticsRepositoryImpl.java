package dev.mariinkys.sociospeix.infrastructure.persistence;

import dev.mariinkys.sociospeix.domain.model.statistics.InterestPopularity;
import dev.mariinkys.sociospeix.domain.repository.StatisticsRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.MemberJpaRepository;
import org.springframework.stereotype.Component;

import java.util.List;

@Component
public class StatisticsRepositoryImpl implements StatisticsRepository {

    private final MemberJpaRepository memberJpaRepository;

    public StatisticsRepositoryImpl(MemberJpaRepository memberJpaRepository) {
        this.memberJpaRepository = memberJpaRepository;
    }

    @Override
    public List<InterestPopularity> countMembersPerInterest() {
        return memberJpaRepository.countMembersPerInterest();
    }
}