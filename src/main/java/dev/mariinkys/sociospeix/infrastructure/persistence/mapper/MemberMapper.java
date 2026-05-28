package dev.mariinkys.sociospeix.infrastructure.persistence.mapper;

import dev.mariinkys.sociospeix.domain.model.Member;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.MemberJpaEntity;
import org.springframework.stereotype.Component;

@Component
public class MemberMapper {

    public Member toDomain(MemberJpaEntity entity) {
        return new Member(
                entity.getId(),
                entity.getName(),
                entity.getSurname(),
                entity.getSecondSurname(),
                entity.getEmail(),
                entity.getBirthdate(),
                entity.getPhone(),
                entity.getNotes(),
                entity.getCreatedAt(),
                entity.getUpdatedAt()
        );
    }

    public MemberJpaEntity toEntity(Member member) {
        return new MemberJpaEntity(
                member.getId(),
                member.getName(),
                member.getSurname(),
                member.getSecondSurname(),
                member.getEmail(),
                member.getBirthdate(),
                member.getPhone(),
                member.getNotes(),
                member.getCreatedAt(),
                member.getUpdatedAt()
        );
    }
}