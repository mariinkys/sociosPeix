package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.application.exception.MemberNotFoundException;
import dev.mariinkys.sociospeix.application.port.MemberUseCase;
import dev.mariinkys.sociospeix.domain.model.Country;
import dev.mariinkys.sociospeix.domain.model.Gender;
import dev.mariinkys.sociospeix.domain.model.Member;
import dev.mariinkys.sociospeix.domain.repository.CountryRepository;
import dev.mariinkys.sociospeix.domain.repository.GenderRepository;
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
    private final GenderRepository genderRepository;
    private final CountryRepository countryRepository;

    public MemberService(MemberRepository memberRepository,
                         GenderRepository genderRepository,
                         CountryRepository countryRepository) {
        this.memberRepository = memberRepository;
        this.genderRepository = genderRepository;
        this.countryRepository = countryRepository;
    }

    @Override
    @Transactional
    public Member createMember(String name, String surname, String secondSurname, String email,
                               String birthdate, String phone, String notes,
                               Integer genderId, Integer countryId) {
        var member = new Member(name, surname, nullToEmpty(secondSurname), email,
                parseDate(birthdate), nullToEmpty(phone), nullToEmpty(notes),
                resolveGender(genderId), resolveCountry(countryId));
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
    public Page<Member> getAllMembers(String search, Pageable pageable) {
        return memberRepository.findAll(search, pageable);
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
                               String email, String birthdate, String phone, String notes,
                               Integer genderId, Integer countryId) {
        Member existing = getMemberById(id);
        return memberRepository.save(
                existing.withUpdatedDetails(name, surname, nullToEmpty(secondSurname),
                        email, parseDate(birthdate), nullToEmpty(phone), nullToEmpty(notes),
                        resolveGender(genderId), resolveCountry(countryId))
        );
    }

    @Override
    @Transactional
    public void deleteMember(UUID id) {
        getMemberById(id);
        memberRepository.deleteById(id);
    }

    // Returns null if no id provided — gender/country are optional
    private Gender resolveGender(Integer id) {
        if (id == null) return null;
        return genderRepository.findById(id)
                .orElseThrow(() -> new IllegalArgumentException("Gender not found with id: " + id));
    }

    private Country resolveCountry(Integer id) {
        if (id == null) return null;
        return countryRepository.findById(id)
                .orElseThrow(() -> new IllegalArgumentException("Country not found with id: " + id));
    }

    private String nullToEmpty(String value) {
        return value == null ? "" : value;
    }

    private LocalDate parseDate(String date) {
        if (date == null || date.isBlank()) return null;
        try {
            return LocalDate.parse(date);
        } catch (DateTimeParseException e) {
            throw new IllegalArgumentException("Invalid date format. Expected yyyy-MM-dd");
        }
    }
}