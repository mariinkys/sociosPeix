package dev.mariinkys.sociospeix.infrastructure.persistence;

import dev.mariinkys.sociospeix.domain.model.Email;
import dev.mariinkys.sociospeix.domain.repository.EmailRepository;
import dev.mariinkys.sociospeix.infrastructure.persistence.entity.EmailJpaEntity;
import dev.mariinkys.sociospeix.infrastructure.persistence.repository.EmailJpaRepository;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Component;

import java.util.Optional;
import java.util.UUID;

@Component
public class EmailRepositoryImpl implements EmailRepository {

    private final EmailJpaRepository jpaRepository;

    public EmailRepositoryImpl(EmailJpaRepository jpaRepository) {
        this.jpaRepository = jpaRepository;
    }

    @Override
    public Email save(Email email) {
        return toDomain(jpaRepository.save(
                new EmailJpaEntity(email.getSubject(), email.getBody(),
                        email.getProvider(), email.getRecipientEmails())
        ));
    }

    @Override
    public Optional<Email> findById(UUID id) {
        return jpaRepository.findById(id).map(this::toDomain);
    }

    @Override
    public Page<Email> findAll(Pageable pageable) {
        return jpaRepository.findAll(pageable).map(this::toDomain);
    }

    @Override
    public Page<Email> findByMember(UUID memberId, Pageable pageable) {
        return jpaRepository.findByMemberId(memberId, pageable).map(this::toDomain);
    }

    private Email toDomain(EmailJpaEntity e) {
        return new Email(e.getId(), e.getSubject(), e.getBody(),
                e.getProvider(), e.getRecipientEmails(), e.getCreatedAt());
    }
}