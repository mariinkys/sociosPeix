package dev.mariinkys.cococms.application.port;

import dev.mariinkys.cococms.domain.model.User;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.UUID;

public interface UserUseCase {
    User createUser(String name, String email);
    User getUserById(UUID id);
    Page<User> getAllUsers(Pageable pageable);
    User updateUser(UUID id, String name, String email);
    void deleteUser(UUID id);
}
