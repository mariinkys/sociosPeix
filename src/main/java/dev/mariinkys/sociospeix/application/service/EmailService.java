package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.application.exception.DailyEmailLimitException;
import dev.mariinkys.sociospeix.application.exception.EmailNotFoundException;
import dev.mariinkys.sociospeix.application.exception.EmailSendException;
import dev.mariinkys.sociospeix.application.exception.MemberNotFoundException;
import dev.mariinkys.sociospeix.application.port.EmailPort;
import dev.mariinkys.sociospeix.application.port.EmailUseCase;
import dev.mariinkys.sociospeix.domain.model.Email;
import dev.mariinkys.sociospeix.domain.model.EmailAttachment;
import dev.mariinkys.sociospeix.domain.model.EmailCategory;
import dev.mariinkys.sociospeix.domain.model.Member;
import dev.mariinkys.sociospeix.domain.repository.EmailRepository;
import dev.mariinkys.sociospeix.domain.repository.EmailSettingsRepository;
import dev.mariinkys.sociospeix.domain.repository.MemberRepository;
import dev.mariinkys.sociospeix.infrastructure.email.EmailProviderRegistry;
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

    private final EmailProviderRegistry providerRegistry;
    private final EmailSettingsRepository emailSettingsRepository;
    private final EmailRepository emailRepository;
    private final MemberRepository memberRepository;

    public EmailService(EmailProviderRegistry providerRegistry,
                        EmailSettingsRepository emailSettingsRepository,
                        EmailRepository emailRepository,
                        MemberRepository memberRepository) {
        this.providerRegistry = providerRegistry;
        this.emailSettingsRepository = emailSettingsRepository;
        this.emailRepository = emailRepository;
        this.memberRepository = memberRepository;
    }

    @Override
    @Transactional
    public Email sendToMember(UUID memberId, String subject, String htmlBody, String previewBody,
                              List<EmailAttachment> attachments) {
        var member = memberRepository.findById(memberId)
                .orElseThrow(() -> new MemberNotFoundException(memberId));

        if (member.getEmail().isBlank()) {
            throw new EmailSendException("Member has no email address");
        }

        return send(subject, htmlBody, previewBody, List.of(member.getEmail()), attachments, EmailCategory.CAMPAIGN);
    }

    @Override
    @Transactional
    public Email sendToAll(String subject, String htmlBody, String previewBody,
                           List<EmailAttachment> attachments) {
        var recipients = resolveRecipientEmails(null);

        if (recipients.isEmpty()) {
            throw new EmailSendException("No members with email addresses found");
        }

        return send(subject, htmlBody, previewBody, recipients, attachments, EmailCategory.CAMPAIGN);
    }

    @Override
    @Transactional
    public Email sendToInterests(List<Integer> interestIds, String subject, String htmlBody, String previewBody,
                                 List<EmailAttachment> attachments) {
        if (interestIds == null || interestIds.isEmpty()) {
            throw new EmailSendException("At least one interest must be specified");
        }

        var recipients = resolveRecipientEmails(interestIds);

        if (recipients.isEmpty()) {
            throw new EmailSendException("No members with email addresses found for the given interests");
        }

        return send(subject, htmlBody, previewBody, recipients, attachments, EmailCategory.CAMPAIGN);
    }

    @Override
    @Transactional(readOnly = true)
    public Page<Email> getAllEmails(Pageable pageable) {
        return emailRepository.findAll(pageable);
    }

    @Override
    @Transactional(readOnly = true)
    public List<Email> getTodayEmails() {
        return emailRepository.findToday();
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

    @Override
    @Transactional(readOnly = true)
    public EmailProviderStatus getProviderStatus() {
        var provider = activeProvider();
        int limit = provider.getDailyLimit();
        int sentToday = emailRepository.countRecipientsToday(provider.getProviderName());
        return new EmailProviderStatus(provider.getProviderName(), limit, sentToday, limit - sentToday);
    }

    @Override
    @Transactional
    public Email sendTransactional(String recipientEmail, String subject, String htmlBody) {
        if (recipientEmail == null || recipientEmail.isBlank()) {
            throw new EmailSendException("Recipient email is required");
        }
        // no cid: images in transactional mail, so the sent and stored bodies are identical
        return send(subject, htmlBody, htmlBody, List.of(recipientEmail), List.of(), EmailCategory.TRANSACTIONAL);
    }

    @Override
    @Transactional(readOnly = true)
    public List<EmailProviderInfo> listAvailableProviders() {
        return providerRegistry.getAll().stream()
                .map(p -> new EmailProviderInfo(p.getProviderName(), p.getDailyLimit()))
                .toList();
    }

    @Override
    @Transactional
    public void setActiveProvider(String providerName) {
        if (!providerRegistry.has(providerName)) {
            throw new EmailSendException("Unknown email provider: " + providerName);
        }
        emailSettingsRepository.setActiveProviderName(providerName);
        log.info("Active email provider switched to {}", providerName);
    }

    @Override
    @Transactional(readOnly = true)
    public MultiEmailCheckResult checkMultiSend(List<Integer> interestIds) {
        var recipients = resolveRecipientEmails(interestIds);
        var provider = activeProvider();

        int limit = provider.getDailyLimit();
        int sentToday = emailRepository.countRecipientsToday(provider.getProviderName());
        int remaining = limit - sentToday;
        int totalRecipients = recipients.size();

        return new MultiEmailCheckResult(
                provider.getProviderName(),
                limit,
                sentToday,
                remaining,
                totalRecipients,
                totalRecipients > remaining
        );
    }

    private List<String> resolveRecipientEmails(List<Integer> interestIds) {
        if (interestIds == null || interestIds.isEmpty()) {
            return memberRepository.findAllMembers().stream()
                    .map(Member::getEmail)
                    .filter(email -> !email.isBlank())
                    .distinct()
                    .toList();
        }

        return memberRepository.findAllByAnyInterestId(interestIds).stream()
                .map(Member::getEmail)
                .filter(email -> !email.isBlank())
                .distinct()
                .toList();
    }

    private Email send(String subject, String htmlBody, String previewBody, List<String> recipients,
                       List<EmailAttachment> attachments, EmailCategory category) {

        var provider = activeProvider();
        checkDailyLimit(provider, recipients.size());

        try {
            provider.send(subject, htmlBody, recipients, attachments);
            log.info("Email sent via {} to {} recipients [{}]",
                    provider.getProviderName(), recipients.size(), category);
        } catch (EmailSendException e) {
            throw e;
        } catch (Exception e) {
            throw new EmailSendException("Failed to send email: " + e.getMessage());
        }

        return emailRepository.save(
                new Email(subject, previewBody, provider.getProviderName(), recipients, category)
        );
    }

    private void checkDailyLimit(EmailPort provider, int requested) {
        int limit = provider.getDailyLimit();
        int sentToday = emailRepository.countRecipientsToday(provider.getProviderName());
        int remaining = limit - sentToday;

        if (requested > remaining) {
            throw new DailyEmailLimitException(
                    provider.getProviderName(), limit, sentToday, requested
            );
        }

        log.debug("Daily limit check passed for {}: {}/{} used, requesting {}",
                provider.getProviderName(), sentToday, limit, requested);
    }

    private EmailPort activeProvider() {
        String name = emailSettingsRepository.getActiveProviderName()
                .orElseThrow(() -> new IllegalStateException("No active email provider configured"));
        return providerRegistry.get(name);
    }
}