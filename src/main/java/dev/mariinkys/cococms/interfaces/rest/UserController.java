package dev.mariinkys.cococms.interfaces.rest;

import dev.mariinkys.cococms.application.port.UserUseCase;
import dev.mariinkys.cococms.application.service.RequesterContext;
import dev.mariinkys.cococms.interfaces.dto.PageResponse;
import dev.mariinkys.cococms.interfaces.dto.auth.RegisterRequest;
import dev.mariinkys.cococms.interfaces.dto.user.UserResponse;
import jakarta.validation.Valid;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.web.bind.annotation.*;

import java.util.Map;
import java.util.Objects;
import java.util.UUID;

@RestController
@RequestMapping("/api/users")
public class UserController {

    private final UserUseCase userUseCase;

    public UserController(UserUseCase userUseCase) {
        this.userUseCase = userUseCase;
    }

    // ADMIN only
    @GetMapping
    @PreAuthorize("hasRole('ADMIN')")
    public ResponseEntity<PageResponse<UserResponse>> getAll(
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size,
            @RequestParam(defaultValue = "createdAt") String sortBy,
            @RequestParam(defaultValue = "desc") String sortDir) {

        var direction = sortDir.equalsIgnoreCase("asc") ? Sort.Direction.ASC : Sort.Direction.DESC;
        var pageable = PageRequest.of(page, size, Sort.by(direction, sortBy));
        var result = userUseCase.getAllUsers(pageable).map(UserResponse::from);
        return ResponseEntity.ok(PageResponse.from(result));
    }

    // ADMIN gets any user, USER gets only themselves (enforced in service)
    @GetMapping("/{id}")
    public ResponseEntity<UserResponse> getById(@PathVariable UUID id,
                                                @AuthenticationPrincipal UserDetails userDetails) {
        var user = userUseCase.getUserById(id, requesterContext(userDetails));
        return ResponseEntity.ok(UserResponse.from(user));
    }

    // ADMIN updates anyone, USER updates only themselves (enforced in service)
    @PutMapping("/{id}")
    public ResponseEntity<UserResponse> update(@PathVariable UUID id,
                                               @Valid @RequestBody RegisterRequest request,
                                               @AuthenticationPrincipal UserDetails userDetails) {
        var user = userUseCase.updateUser(id, request.name(), request.email(), requesterContext(userDetails));
        return ResponseEntity.ok(UserResponse.from(user));
    }

    // ADMIN deletes anyone, USER deletes only themselves (enforced in service)
    @DeleteMapping("/{id}")
    public ResponseEntity<Void> delete(@PathVariable UUID id,
                                       @AuthenticationPrincipal UserDetails userDetails) {
        userUseCase.deleteUser(id, requesterContext(userDetails));
        return ResponseEntity.noContent().build();
    }

    @GetMapping("/me")
    public Object me(@AuthenticationPrincipal UserDetails userDetails) {
        return Map.of(
                "username", userDetails.getUsername(),
                "authorities", userDetails.getAuthorities()
        );
    }

    private RequesterContext requesterContext(UserDetails userDetails) {
        boolean isAdmin = userDetails.getAuthorities().stream()
                .anyMatch(a -> Objects.equals(a.getAuthority(), "ROLE_ADMIN"));
        return new RequesterContext(userDetails.getUsername(), isAdmin);
    }
}
