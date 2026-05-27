package dev.mariinkys.cococms.interfaces.rest;

import dev.mariinkys.cococms.application.port.UserUseCase;
import dev.mariinkys.cococms.application.service.RefreshTokenService;
import dev.mariinkys.cococms.infrastructure.security.JwtService;
import dev.mariinkys.cococms.interfaces.dto.auth.AuthRequest;
import dev.mariinkys.cococms.interfaces.dto.auth.AuthResponse;
import dev.mariinkys.cococms.interfaces.dto.auth.RefreshRequest;
import dev.mariinkys.cococms.interfaces.dto.auth.RegisterRequest;
import jakarta.validation.Valid;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.context.SecurityContextHolder;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/auth")
public class AuthController {

    private final AuthenticationManager authManager;
    private final JwtService jwtService;
    private final RefreshTokenService refreshTokenService;
    private final UserUseCase userUseCase;

    public AuthController(AuthenticationManager authManager,
                          JwtService jwtService,
                          RefreshTokenService refreshTokenService,
                          UserUseCase userUseCase) {
        this.authManager = authManager;
        this.jwtService = jwtService;
        this.refreshTokenService = refreshTokenService;
        this.userUseCase = userUseCase;
    }

    @PostMapping("/register")
    public ResponseEntity<AuthResponse> register(@Valid @RequestBody RegisterRequest request) {
        userUseCase.createUser(request.name(), request.email(), request.password());
        return ResponseEntity
                .status(HttpStatus.CREATED)
                .body(issueTokenPair(request.email(), request.password()));
    }

    @PostMapping("/login")
    public ResponseEntity<AuthResponse> login(@Valid @RequestBody AuthRequest request) {
        return ResponseEntity.ok(issueTokenPair(request.email(), request.password()));
    }

    @PostMapping("/refresh")
    public ResponseEntity<AuthResponse> refresh(@Valid @RequestBody RefreshRequest request) {
        // validateAndRotate revokes the old token and returns the owner's email
        String email = refreshTokenService.validateAndRotate(request.refreshToken());

        String newAccessToken = jwtService.generateToken(email);
        String newRefreshToken = refreshTokenService.createRefreshToken(email);

        return ResponseEntity.ok(new AuthResponse(newAccessToken, newRefreshToken));
    }

    @PostMapping("/logout")
    public ResponseEntity<Void> logout() {
        var authentication = SecurityContextHolder.getContext().getAuthentication();

        if (authentication == null || !authentication.isAuthenticated()) {
            return ResponseEntity.status(HttpStatus.UNAUTHORIZED).build();
        }

        refreshTokenService.revokeAllForUser(authentication.getName());
        return ResponseEntity.noContent().build();
    }

    // Shared logic: authenticate + issue both tokens
    private AuthResponse issueTokenPair(String email, String rawPassword) {
        authManager.authenticate(
                new UsernamePasswordAuthenticationToken(email, rawPassword)
        );
        String accessToken = jwtService.generateToken(email);
        String refreshToken = refreshTokenService.createRefreshToken(email);
        return new AuthResponse(accessToken, refreshToken);
    }
}