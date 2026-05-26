package dev.mariinkys.cococms.interfaces.dto;

public class AuthRequest {
    private String email;
    private String password;

    public AuthRequest() {}
    public String getEmail() { return email; }
    public String getPassword() { return password; }
}