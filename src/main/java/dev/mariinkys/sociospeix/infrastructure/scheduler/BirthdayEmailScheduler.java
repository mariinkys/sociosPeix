package dev.mariinkys.sociospeix.infrastructure.scheduler;

import dev.mariinkys.sociospeix.application.service.BirthdayEmailService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Component;

@Component
public class BirthdayEmailScheduler {

    private static final Logger log = LoggerFactory.getLogger(BirthdayEmailScheduler.class);

    private final BirthdayEmailService birthdayEmailService;

    public BirthdayEmailScheduler(BirthdayEmailService birthdayEmailService) {
        this.birthdayEmailService = birthdayEmailService;
    }

    // 8:00 AM Madrid time
    @Scheduled(cron = "0 0 8 * * *", zone = "Europe/Madrid")
    public void run() {
        log.info("Birthday scheduler triggered");
        birthdayEmailService.sendBirthdayEmails();
    }
}