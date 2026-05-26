package dev.mariinkys.cococms.domain.repository;

import dev.mariinkys.cococms.domain.model.User;
import java.util.Optional;
import java.util.UUID;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

public interface UserRepository {
    User save(User user);
    Optional<User> findById(UUID id);
    Page<User> findAll(Pageable pageable);
    void deleteById(UUID id);
    boolean existsByEmail(String email);
}