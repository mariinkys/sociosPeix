package dev.mariinkys.sociospeix.infrastructure.config;

import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.stereotype.Component;

@Component
@ConfigurationProperties(prefix = "email")
public class EmailProperties {
    private String from;
    private Resend resend = new Resend();

    public String getFrom() { return from; }
    public void setFrom(String from) { this.from = from; }
    public Resend getResend() { return resend; }
    public void setResend(Resend resend) { this.resend = resend; }

    public static class Resend {
        private String apiKey;
        public String getApiKey() { return apiKey; }
        public void setApiKey(String apiKey) { this.apiKey = apiKey; }
    }
}