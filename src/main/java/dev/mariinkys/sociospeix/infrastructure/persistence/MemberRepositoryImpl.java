package dev.mariinkys.sociospeix.infrastructure.persistence;

import dev.mariinkys.sociospeix.domain.model.Member;
import dev.mariinkys.sociospeix.domain.repository.MemberRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.CountryJpaEntity;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.GenderJpaEntity;
import dev.mariinkys.sociospeix.infrastructure.persistence.mapper.MemberMapper;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.CountryJpaRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.GenderJpaRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.MemberJpaRepository;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Component
public class MemberRepositoryImpl implements MemberRepository {

    private final MemberJpaRepository jpaRepository;
    private final GenderJpaRepository genderJpaRepository;
    private final CountryJpaRepository countryJpaRepository;
    private final MemberMapper mapper;

    public MemberRepositoryImpl(MemberJpaRepository jpaRepository,
                                GenderJpaRepository genderJpaRepository,
                                CountryJpaRepository countryJpaRepository,
                                MemberMapper mapper) {
        this.jpaRepository = jpaRepository;
        this.genderJpaRepository = genderJpaRepository;
        this.countryJpaRepository = countryJpaRepository;
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
        return jpaRepository.findById(id).map(mapper::toDomain);
    }

    @Override
    public Page<Member> findAll(Pageable pageable) {
        return jpaRepository.findAll(pageable).map(mapper::toDomain);
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
}