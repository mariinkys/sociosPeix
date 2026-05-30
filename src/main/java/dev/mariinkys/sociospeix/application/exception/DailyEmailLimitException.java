package dev.mariinkys.sociospeix.application.exception;

public class DailyEmailLimitException extends RuntimeException {
    public DailyEmailLimitException(String provider, int limit, int current, int requested) {
        super(String.format(
                "Daily send limit reached for provider '%s': limit is %d, already sent %d today, " +
                        "attempted to send to %d more recipients",
                provider, limit, current, requested
        ));
    }
}
