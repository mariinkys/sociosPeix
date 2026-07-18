package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.application.exception.DailyEmailLimitException;
import dev.mariinkys.sociospeix.application.port.EmailUseCase;
import dev.mariinkys.sociospeix.domain.model.Member;
import dev.mariinkys.sociospeix.domain.repository.EmailRepository;
import dev.mariinkys.sociospeix.domain.repository.MemberRepository;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.LocalDate;
import java.util.List;

@Service
public class BirthdayEmailService {

    private static final Logger log = LoggerFactory.getLogger(BirthdayEmailService.class);
    private static final String SUBJECT = "¡Feliz Cumpleaños! 🎂";

    private final MemberRepository memberRepository;
    private final EmailRepository emailRepository;
    private final EmailUseCase emailUseCase;
    private final BirthdayEmailTemplate template;

    public BirthdayEmailService(MemberRepository memberRepository,
                                EmailRepository emailRepository,
                                EmailUseCase emailUseCase,
                                BirthdayEmailTemplate template) {
        this.memberRepository = memberRepository;
        this.emailRepository = emailRepository;
        this.emailUseCase = emailUseCase;
        this.template = template;
    }

    @Transactional
    public void sendBirthdayEmails() {
        LocalDate today = LocalDate.now();
        log.info("Birthday email job started for {}", today);

        List<Member> birthdayMembers = memberRepository.findByBirthday(
                today.getDayOfMonth(), today.getMonthValue()
        );

        if (birthdayMembers.isEmpty()) {
            log.info("No birthday members today");
            return;
        }

        log.info("Found {} member(s) with birthdays today", birthdayMembers.size());

        int sent = 0, skipped = 0, failed = 0;

        for (Member member : birthdayMembers) {
            try {
                if (shouldSkip(member)) {
                    skipped++;
                    continue;
                }
                sendTo(member);
                sent++;
            } catch (DailyEmailLimitException e) {
                log.warn("Daily limit reached after {} birthday emails. {} members missed.",
                        sent, birthdayMembers.size() - sent - skipped);
                break;
            } catch (Exception e) {
                log.error("Failed birthday email for member {}: {}", member.getId(), e.getMessage());
                failed++;
            }
        }

        log.info("Birthday job done — sent: {}, skipped: {}, failed: {}", sent, skipped, failed);
    }

    private boolean shouldSkip(Member member) {
        if (member.getEmail().isBlank()) {
            log.debug("Skipping member {} — no email address", member.getId());
            return true;
        }
        if (emailRepository.alreadySentTodayTo(member.getEmail(), SUBJECT)) {
            log.debug("Skipping member {} — already sent today", member.getId());
            return true;
        }
        return false;
    }

    private void sendTo(Member member) {
        String html = template.build(member.getName());
        var attachments = List.of(template.logoAttachment());

        // sendToMember resolves whichever provider is currently active and enforces its daily limit
        emailUseCase.sendToMember(member.getId(), SUBJECT, html, attachments);

        log.info("Birthday email sent to member {} ({})", member.getId(), member.getEmail());
    }
}