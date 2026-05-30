package dev.mariinkys.sociospeix.domain.repository;

import dev.mariinkys.sociospeix.domain.model.Email;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.Optional;
import java.util.UUID;

public interface EmailRepository {
    Email save(Email email);
    Optional<Email> findById(UUID id);
    Page<Email> findAll(Pageable pageable);
    Page<Email> findByMember(UUID memberId, Pageable pageable);
    int countRecipientsToday(String provider);
}