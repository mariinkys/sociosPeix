package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.application.exception.EmailNotFoundException;
import dev.mariinkys.sociospeix.application.exception.EmailSendException;
import dev.mariinkys.sociospeix.application.exception.MemberNotFoundException;
import dev.mariinkys.sociospeix.application.port.EmailPort;
import dev.mariinkys.sociospeix.application.port.EmailUseCase;
import dev.mariinkys.sociospeix.domain.model.Email;
import dev.mariinkys.sociospeix.domain.model.EmailAttachment;
import dev.mariinkys.sociospeix.domain.repository.EmailRepository;
import dev.mariinkys.sociospeix.domain.repository.MemberRepository;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.UUID;

@Service
public class EmailService implements EmailUseCase {

    private static final Logger log = LoggerFactory.getLogger(EmailService.class);

    private final EmailPort emailPort;
    private final EmailRepository emailRepository;
    private final MemberRepository memberRepository;

    public EmailService(EmailPort emailPort,
                        EmailRepository emailRepository,
                        MemberRepository memberRepository) {
        this.emailPort = emailPort;
        this.emailRepository = emailRepository;
        this.memberRepository = memberRepository;
    }

    @Override
    @Transactional
    public Email sendToMember(UUID memberId, String subject, String htmlBody,
                              List<EmailAttachment> attachments) {
        var member = memberRepository.findById(memberId)
                .orElseThrow(() -> new MemberNotFoundException(memberId));

        if (member.getEmail().isBlank()) {
            throw new EmailSendException("Member has no email address");
        }

        return send(subject, htmlBody, List.of(member.getEmail()), attachments);
    }

    @Override
    @Transactional
    public Email sendToAll(String subject, String htmlBody, List<EmailAttachment> attachments) {
        var recipients = memberRepository.findAllMembers().stream()
                .map(dev.mariinkys.sociospeix.domain.model.Member::getEmail)
                .filter(email -> !email.isBlank())
                .distinct()
                .toList();

        if (recipients.isEmpty()) {
            throw new EmailSendException("No members with email addresses found");
        }

        return send(subject, htmlBody, recipients, attachments);
    }

    @Override
    @Transactional
    public Email sendToInterests(List<Integer> interestIds, String subject, String htmlBody,
                                 List<EmailAttachment> attachments) {
        if (interestIds == null || interestIds.isEmpty()) {
            throw new EmailSendException("At least one interest must be specified");
        }

        var recipients = memberRepository.findAllByAnyInterestId(interestIds).stream()
                .map(dev.mariinkys.sociospeix.domain.model.Member::getEmail)
                .filter(email -> !email.isBlank())
                .distinct()
                .toList();

        if (recipients.isEmpty()) {
            throw new EmailSendException("No members with email addresses found for the given interests");
        }

        return send(subject, htmlBody, recipients, attachments);
    }

    @Override
    @Transactional(readOnly = true)
    public Page<Email> getAllEmails(Pageable pageable) {
        return emailRepository.findAll(pageable);
    }

    @Override
    @Transactional(readOnly = true)
    public Page<Email> getAllEmailsByMember(UUID memberId, Pageable pageable) {
        return emailRepository.findByMember(memberId, pageable);
    }

    @Override
    @Transactional(readOnly = true)
    public Email getEmailById(UUID id) {
        return emailRepository.findById(id)
                .orElseThrow(() -> new EmailNotFoundException(id));
    }

    // sends via the provider and persists the record
    private Email send(String subject, String htmlBody,
                       List<String> recipients, List<EmailAttachment> attachments) {
        try {
            emailPort.send(subject, htmlBody, recipients, attachments);
            log.info("Email sent via {} to {} recipients", emailPort.getProviderName(), recipients.size());
        } catch (EmailSendException e) {
            throw e;
        } catch (Exception e) {
            throw new EmailSendException("Failed to send email: " + e.getMessage());
        }

        return emailRepository.save(
                new Email(subject, htmlBody, emailPort.getProviderName(), recipients)
        );
    }
}