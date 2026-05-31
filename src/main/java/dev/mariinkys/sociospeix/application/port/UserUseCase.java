package dev.mariinkys.sociospeix.application.port;

import dev.mariinkys.sociospeix.application.service.RequesterContext;
import dev.mariinkys.sociospeix.domain.model.Role;
import dev.mariinkys.sociospeix.domain.model.User;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.UUID;

public interface UserUseCase {
    void createUser(String name, String email, String rawPassword);
    User getUserById(UUID id, RequesterContext requester);
    User getUserByEmail(String email);
    Page<User> getAllUsers(String search, Pageable pageable);
    User updateUser(UUID id, String name, String email, RequesterContext requester);
    User updateUserRole(UUID id, Role role);
    void changePassword(UUID id, String currentPassword, String newPassword, RequesterContext requester);
    void deleteUser(UUID id, RequesterContext requester);
}
