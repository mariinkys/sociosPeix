package dev.mariinkys.sociospeix.interfaces.dto.member;

import dev.mariinkys.sociospeix.domain.model.Member;

import java.time.LocalDate;
import java.time.LocalDateTime;
import java.util.UUID;

public record MemberResponse(
        UUID id,
        String name,
        String surname,
        String secondSurname,
        String fullName,
        String email,
        LocalDate birthdate,
        String phone,
        String notes,
        LocalDateTime createdAt
) {
    public static MemberResponse from(Member member) {
        return new MemberResponse(
                member.getId(),
                member.getName(),
                member.getSurname(),
                member.getSecondSurname(),
                member.getFullName(),
                member.getEmail(),
                member.getBirthdate(),
                member.getPhone(),
                member.getNotes(),
                member.getCreatedAt()
        );
    }
}
