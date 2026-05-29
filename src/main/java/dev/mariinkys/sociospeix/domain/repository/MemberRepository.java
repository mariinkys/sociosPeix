package dev.mariinkys.sociospeix.domain.repository;

import dev.mariinkys.sociospeix.domain.model.Member;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

public interface MemberRepository {
    Member save(Member member);
    Optional<Member> findById(UUID id);
    Page<Member> findAll(String search, List<Integer> interestIds, Pageable pageable);
    List<Member> findByBirthday(int day, int month);
    void deleteById(UUID id);

    void syncInterests(UUID memberId, List<Integer> interestIds);
}
