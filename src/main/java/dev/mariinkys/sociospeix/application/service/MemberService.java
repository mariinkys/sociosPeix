package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.application.exception.MemberNotFoundException;
import dev.mariinkys.sociospeix.application.port.MemberUseCase;
import dev.mariinkys.sociospeix.domain.model.Member;
import dev.mariinkys.sociospeix.domain.repository.MemberRepository;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.time.LocalDate;
import java.time.format.DateTimeParseException;
import java.util.List;
import java.util.UUID;

@Service
public class MemberService implements MemberUseCase {

    private final MemberRepository memberRepository;

    public MemberService(MemberRepository memberRepository) {
        this.memberRepository = memberRepository;
    }

    @Override
    @Transactional
    public Member createMember(String name, String surname, String secondSurname,
                               String email, String birthdate, String phone, String notes) {
        var member = new Member(
                name,
                surname,
                nullToEmpty(secondSurname),
                email,
                parseDate(birthdate),
                nullToEmpty(phone),
                nullToEmpty(notes)
        );
        return memberRepository.save(member);
    }

    @Override
    @Transactional(readOnly = true)
    public Member getMemberById(UUID id) {
        return memberRepository.findById(id)
                .orElseThrow(() -> new MemberNotFoundException(id));
    }

    @Override
    @Transactional(readOnly = true)
    public Page<Member> getAllMembers(Pageable pageable) {
        return memberRepository.findAll(pageable);
    }

    @Override
    @Transactional(readOnly = true)
    public List<Member> getTodaysBirthdays() {
        LocalDate today = LocalDate.now();
        return memberRepository.findByBirthday(today.getDayOfMonth(), today.getMonthValue());
    }

    @Override
    @Transactional
    public Member updateMember(UUID id, String name, String surname, String secondSurname,
                               String email, String birthdate, String phone, String notes) {
        Member existing = getMemberById(id);
        return memberRepository.save(
                existing.withUpdatedDetails(name, surname, nullToEmpty(secondSurname),
                        email, parseDate(birthdate), nullToEmpty(phone), nullToEmpty(notes))
        );
    }

    @Override
    @Transactional
    public void deleteMember(UUID id) {
        getMemberById(id); // throws if not found
        memberRepository.deleteById(id);
    }

    // Null-safe date parser, birthdate is optional
    private LocalDate parseDate(String date) {
        if (date == null || date.isBlank()) return null;
        try {
            return LocalDate.parse(date);
        } catch (DateTimeParseException e) {
            throw new IllegalArgumentException("Invalid date format. Expected yyyy-MM-dd");
        }
    }

    private String nullToEmpty(String value) {
        return value == null ? "" : value;
    }
}