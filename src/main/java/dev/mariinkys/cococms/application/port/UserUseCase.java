package dev.mariinkys.cococms.application.port;

import dev.mariinkys.cococms.application.service.RequesterContext;
import dev.mariinkys.cococms.domain.model.User;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.UUID;

public interface UserUseCase {
    void createUser(String name, String email, String rawPassword);
    User getUserById(UUID id, RequesterContext requester);
    Page<User> getAllUsers(Pageable pageable);
    User updateUser(UUID id, String name, String email, RequesterContext requester);
    void deleteUser(UUID id, RequesterContext requester);
}
