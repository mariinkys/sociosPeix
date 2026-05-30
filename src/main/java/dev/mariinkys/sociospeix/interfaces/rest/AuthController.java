package dev.mariinkys.sociospeix.interfaces.rest;

import dev.mariinkys.sociospeix.application.port.UserUseCase;
import dev.mariinkys.sociospeix.application.service.RefreshTokenService;
import dev.mariinkys.sociospeix.domain.model.User;
import dev.mariinkys.sociospeix.infrastructure.security.cookie.CookieService;
import dev.mariinkys.sociospeix.infrastructure.security.jwt.JwtService;
import dev.mariinkys.sociospeix.application.service.LoginAttemptService;
import dev.mariinkys.sociospeix.interfaces.dto.auth.AuthRequest;
import dev.mariinkys.sociospeix.interfaces.dto.auth.AuthResponse;
import dev.mariinkys.sociospeix.interfaces.dto.auth.RegisterRequest;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import jakarta.validation.Valid;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.BadCredentialsException;
import org.springframework.security.authentication.LockedException;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/auth")
public class AuthController {

    private final AuthenticationManager authManager;
    private final JwtService jwtService;
    private final RefreshTokenService refreshTokenService;
    private final UserUseCase userUseCase;
    private final CookieService cookieService;
    private final LoginAttemptService loginAttemptService;

    public AuthController(AuthenticationManager authManager,
                          JwtService jwtService,
                          RefreshTokenService refreshTokenService,
                          UserUseCase userUseCase,
                          CookieService cookieService,
                          LoginAttemptService loginAttemptService) {
        this.authManager = authManager;
        this.jwtService = jwtService;
        this.refreshTokenService = refreshTokenService;
        this.userUseCase = userUseCase;
        this.cookieService = cookieService;
        this.loginAttemptService = loginAttemptService;
    }

    @PostMapping("/register")
    @PreAuthorize("hasRole('ADMIN')")
    public ResponseEntity<AuthResponse> register(@Valid @RequestBody RegisterRequest request,
                                                 HttpServletResponse response) {
        userUseCase.createUser(request.name(), request.email(), request.password());
        return ResponseEntity
                .status(HttpStatus.CREATED)
                .body(issueTokensAndBuildResponse(request.email(), request.password(), response));
    }

    @PostMapping("/login")
    public ResponseEntity<AuthResponse> login(@Valid @RequestBody AuthRequest request,
                                              HttpServletResponse response) {
        return ResponseEntity.ok(issueTokensAndBuildResponse(request.email(), request.password(), response));
    }

    @PostMapping("/refresh")
    public ResponseEntity<AuthResponse> refresh(HttpServletRequest request,
                                                HttpServletResponse response) {
        String refreshToken = extractRefreshTokenFromCookie(request);
        if (refreshToken == null) {
            return ResponseEntity.status(HttpStatus.UNAUTHORIZED).build();
        }

        String email = refreshTokenService.validateAndRotate(refreshToken);
        User user = userUseCase.getUserByEmail(email);

        setTokenCookies(email, response);

        return ResponseEntity.ok(AuthResponse.from(user));
    }

    @PostMapping("/logout")
    public ResponseEntity<Void> logout(@AuthenticationPrincipal UserDetails userDetails,
                                       HttpServletResponse response) {
        refreshTokenService.revokeAllForUser(userDetails.getUsername());
        response.addHeader(HttpHeaders.SET_COOKIE, cookieService.clearAccessTokenCookie().toString());
        response.addHeader(HttpHeaders.SET_COOKIE, cookieService.clearRefreshTokenCookie().toString());
        return ResponseEntity.noContent().build();
    }

    // Authenticates, sets cookies, returns user info shared by login and register
    private AuthResponse issueTokensAndBuildResponse(String email, String rawPassword,
                                                     HttpServletResponse response) {
        try {
            authManager.authenticate(new UsernamePasswordAuthenticationToken(email, rawPassword));
            loginAttemptService.onSuccess(email);
        }catch (LockedException e) {
            System.out.println("User " + email + " is locked");
            throw e; // handled by GlobalExceptionHandler
        }  catch (BadCredentialsException e) {
            loginAttemptService.onFailure(email);
            throw e;
        }

        User user = userUseCase.getUserByEmail(email);
        setTokenCookies(email, response);
        return AuthResponse.from(user);
    }

    private void setTokenCookies(String email, HttpServletResponse response) {
        String accessToken = jwtService.generateToken(email);
        String refreshToken = refreshTokenService.createRefreshToken(email);
        response.addHeader(HttpHeaders.SET_COOKIE, cookieService.createAccessTokenCookie(accessToken).toString());
        response.addHeader(HttpHeaders.SET_COOKIE, cookieService.createRefreshTokenCookie(refreshToken).toString());
    }

    private String extractRefreshTokenFromCookie(HttpServletRequest request) {
        if (request.getCookies() == null) return null;
        for (var cookie : request.getCookies()) {
            if ("refresh_token".equals(cookie.getName())) return cookie.getValue();
        }
        return null;
    }
}