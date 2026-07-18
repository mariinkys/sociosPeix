package dev.mariinkys.sociospeix.infrastructure.config;

import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.stereotype.Component;

import java.util.HashMap;
import java.util.Map;

@Component
@ConfigurationProperties(prefix = "email")
public class EmailProperties {
    private String from;
    private Map<String, ProviderProperties> providers = new HashMap<>();

    public String getFrom() { return from; }
    public void setFrom(String from) { this.from = from; }
    public Map<String, ProviderProperties> getProviders() { return providers; }
    public void setProviders(Map<String, ProviderProperties> providers) { this.providers = providers; }

    public ProviderProperties forProvider(String name) {
        return providers.getOrDefault(name.toLowerCase(), new ProviderProperties());
    }

    public static class ProviderProperties {
        private String apiKey;
        private int dailyLimit = 100; // safe default

        public String getApiKey() { return apiKey; }
        public void setApiKey(String apiKey) { this.apiKey = apiKey; }
        public int getDailyLimit() { return dailyLimit; }
        public void setDailyLimit(int dailyLimit) { this.dailyLimit = dailyLimit; }

        public boolean isConfigured() {
            return apiKey != null && !apiKey.isBlank() && apiKey.length() > 5;
        }
    }
}