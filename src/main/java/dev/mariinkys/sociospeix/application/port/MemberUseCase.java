package dev.mariinkys.sociospeix.application.port;

import dev.mariinkys.sociospeix.domain.model.Member;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.List;
import java.util.UUID;

public interface MemberUseCase {
    Member createMember(String name, String surname, String secondSurname, String email,
                        String birthdate, String phone, String notes,
                        Integer genderId, Integer countryId, List<Integer> interestIds);
    Member getMemberById(UUID id);
    Page<Member> getAllMembers(String search, List<Integer> interestIds, Pageable pageable);
    List<Member> getTodaysBirthdays();
    Member updateMember(UUID id, String name, String surname, String secondSurname,
                        String email, String birthdate, String phone, String notes,
                        Integer genderId, Integer countryId, List<Integer> interestIds);
    void deleteMember(UUID id);
}
