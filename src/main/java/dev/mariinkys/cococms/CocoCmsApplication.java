package dev.mariinkys.cococms;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.scheduling.annotation.EnableScheduling;

@SpringBootApplication
@EnableScheduling
public class CocoCmsApplication {
    static void main(String[] args) {
        SpringApplication.run(CocoCmsApplication.class, args);
    }
}
