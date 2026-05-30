package dev.mariinkys.sociospeix.infrastructure.persistence;

import dev.mariinkys.sociospeix.domain.model.Interest;
import dev.mariinkys.sociospeix.domain.model.Member;
import dev.mariinkys.sociospeix.domain.repository.MemberRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.CountryJpaEntity;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.GenderJpaEntity;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.InterestJpaEntity;
import dev.mariinkys.sociospeix.infrastructure.persistence.mapper.MemberMapper;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.CountryJpaRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.GenderJpaRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.InterestJpaRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.MemberJpaRepository;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Component;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Component
public class MemberRepositoryImpl implements MemberRepository {

    private final MemberJpaRepository jpaRepository;
    private final GenderJpaRepository genderJpaRepository;
    private final CountryJpaRepository countryJpaRepository;
    private final InterestJpaRepository interestJpaRepository;
    private final MemberMapper mapper;

    public MemberRepositoryImpl(MemberJpaRepository jpaRepository,
                                GenderJpaRepository genderJpaRepository,
                                CountryJpaRepository countryJpaRepository,
                                InterestJpaRepository interestJpaRepository,
                                MemberMapper mapper) {
        this.jpaRepository = jpaRepository;
        this.genderJpaRepository = genderJpaRepository;
        this.countryJpaRepository = countryJpaRepository;
        this.interestJpaRepository = interestJpaRepository;
        this.mapper = mapper;
    }

    @Override
    public Member save(Member member) {
        GenderJpaEntity genderEntity = member.getGender() != null
                ? genderJpaRepository.getReferenceById(member.getGender().getId())
                : null;
        CountryJpaEntity countryEntity = member.getCountry() != null
                ? countryJpaRepository.getReferenceById(member.getCountry().getId())
                : null;

        return mapper.toDomain(jpaRepository.save(mapper.toEntity(member, genderEntity, countryEntity)));
    }

    @Override
    public Optional<Member> findById(UUID id) {
        return jpaRepository.findById(id).map(entity -> {
            var interests = interestJpaRepository.findByMemberId(id)
                    .stream()
                    .map(e -> new Interest(e.getId(), e.getName(), e.getDescription()))
                    .toList();
            return mapper.toDomain(entity).withInterests(interests);
        });
    }

    @Override
    public Page<Member> findAll(String search, List<Integer> interestIds, Pageable pageable) {
        List<Integer> ids = interestIds == null ? List.of() : interestIds;
        return jpaRepository.findAll(search, ids, ids.size(), pageable).map(entity -> {
            var interests = interestJpaRepository.findByMemberId(entity.getId())
                    .stream()
                    .map(e -> new Interest(e.getId(), e.getName(), e.getDescription()))
                    .toList();
            return mapper.toDomain(entity).withInterests(interests);
        });
    }

    @Override
    public List<Member> findByBirthday(int day, int month) {
        return jpaRepository.findByBirthday(day, month)
                .stream()
                .map(mapper::toDomain)
                .toList();
    }

    @Override
    public void deleteById(UUID id) {
        jpaRepository.deleteById(id);
    }

    @Override
    public void syncInterests(UUID memberId, List<Integer> interestIds) {
        var member = jpaRepository.findById(memberId).orElseThrow();
        var interests = interestIds.isEmpty()
                ? List.<InterestJpaEntity>of()
                : interestJpaRepository.findAllById(interestIds);

        member.setInterests(new ArrayList<>(interests));
        jpaRepository.save(member);
    }

    @Override
    public List<Member> findAllMembers() {
        return jpaRepository.findAllMembers().stream()
                .map(entity -> {
                    var interests = interestJpaRepository.findByMemberId(entity.getId())
                            .stream()
                            .map(e -> new Interest(e.getId(), e.getName(), e.getDescription()))
                            .toList();
                    return mapper.toDomain(entity).withInterests(interests);
                })
                .toList();
    }

    @Override
    public List<Member> findAllByAnyInterestId(List<Integer> interestIds) {
        return jpaRepository.findAllByAnyInterestId(interestIds).stream()
                .map(entity -> {
                    var interests = interestJpaRepository.findByMemberId(entity.getId())
                            .stream()
                            .map(e -> new Interest(e.getId(), e.getName(), e.getDescription()))
                            .toList();
                    return mapper.toDomain(entity).withInterests(interests);
                })
                .toList();
    }
}