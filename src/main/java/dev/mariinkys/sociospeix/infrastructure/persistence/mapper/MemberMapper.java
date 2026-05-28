package dev.mariinkys.sociospeix.infrastructure.persistence.mapper;

import dev.mariinkys.sociospeix.domain.model.Country;
import dev.mariinkys.sociospeix.domain.model.Gender;
import dev.mariinkys.sociospeix.domain.model.Member;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.CountryJpaEntity;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.GenderJpaEntity;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.MemberJpaEntity;
import org.springframework.stereotype.Component;

@Component
public class MemberMapper {

    public Member toDomain(MemberJpaEntity entity) {
        Gender gender = entity.getGender() != null
                ? new Gender(entity.getGender().getId(), entity.getGender().getName())
                : null;
        Country country = entity.getCountry() != null
                ? new Country(entity.getCountry().getId(), entity.getCountry().getName())
                : null;

        return new Member(
                entity.getId(), entity.getName(), entity.getSurname(),
                entity.getSecondSurname(), entity.getEmail(), entity.getBirthdate(),
                entity.getPhone(), entity.getNotes(), gender, country,
                entity.getCreatedAt(), entity.getUpdatedAt()
        );
    }

    public MemberJpaEntity toEntity(Member member,
                                    GenderJpaEntity genderEntity,
                                    CountryJpaEntity countryEntity) {
        return new MemberJpaEntity(
                member.getId(), member.getName(), member.getSurname(),
                member.getSecondSurname(), member.getEmail(), member.getBirthdate(),
                member.getPhone(), member.getNotes(), genderEntity, countryEntity,
                member.getCreatedAt(), member.getUpdatedAt()
        );
    }
}