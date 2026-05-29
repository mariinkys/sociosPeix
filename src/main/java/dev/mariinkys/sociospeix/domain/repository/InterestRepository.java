package dev.mariinkys.sociospeix.domain.repository;

import dev.mariinkys.sociospeix.domain.model.Interest;
import java.util.List;
import java.util.Optional;
import java.util.UUID;

public interface InterestRepository {
    Interest save(Interest interest);
    Optional<Interest> findById(Integer id);
    List<Interest> findAll();
    List<Interest> findByMemberId(UUID memberId);
    void deleteById(Integer id);
}
