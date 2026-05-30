package dev.mariinkys.sociospeix.application.exception;

public class EmailSendException extends RuntimeException {
  public EmailSendException(String message) {
    super(message);
  }
}
