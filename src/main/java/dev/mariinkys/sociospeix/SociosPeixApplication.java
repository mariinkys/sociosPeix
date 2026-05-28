package dev.mariinkys.sociospeix;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.scheduling.annotation.EnableScheduling;

@SpringBootApplication
@EnableScheduling
public class SociosPeixApplication {
    static void main(String[] args) {
        SpringApplication.run(SociosPeixApplication.class, args);
    }
}
