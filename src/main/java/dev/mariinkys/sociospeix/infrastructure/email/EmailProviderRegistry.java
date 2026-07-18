package dev.mariinkys.sociospeix.infrastructure.email;

import dev.mariinkys.sociospeix.application.port.EmailPort;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.Map;
import java.util.function.Function;
import java.util.stream.Collectors;

@Component
public class EmailProviderRegistry {

    private final Map<String, EmailPort> providersByName;

    // Spring injects EVERY bean implementing EmailPort here automatically -
    // asking for a List sidesteps the "ambiguous bean" error you'd get
    // asking for a single EmailPort once more than one @Component exists.
    public EmailProviderRegistry(List<EmailPort> providers) {
        this.providersByName = providers.stream()
                .collect(Collectors.toMap(EmailPort::getProviderName, Function.identity()));
    }

    public EmailPort get(String providerName) {
        var provider = providersByName.get(providerName);
        if (provider == null) {
            throw new IllegalArgumentException("Unknown email provider: " + providerName);
        }
        return provider;
    }

    public boolean has(String providerName) {
        return providersByName.containsKey(providerName);
    }

    public List<EmailPort> getAll() {
        return providersByName.values().stream().toList();
    }
}