package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.application.exception.EmailAlreadyInUseException;
import dev.mariinkys.sociospeix.application.exception.UserNotFoundException;
import dev.mariinkys.sociospeix.application.port.PasswordHasher;
import dev.mariinkys.sociospeix.domain.model.Role;
import dev.mariinkys.sociospeix.domain.model.User;
import dev.mariinkys.sociospeix.domain.repository.UserRepository;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.security.access.AccessDeniedException;
import org.springframework.stereotype.Service;

import java.util.UUID;
import dev.mariinkys.sociospeix.application.port.UserUseCase;
import org.springframework.transaction.annotation.Transactional;

@Service
public class UserService implements UserUseCase {

    private final UserRepository userRepository;
    private final PasswordHasher passwordHasher;

    public UserService(UserRepository userRepository, PasswordHasher passwordHasher) {
        this.userRepository = userRepository;
        this.passwordHasher = passwordHasher;
    }

    @Override
    @Transactional
    public void createUser(String name, String email, String rawPassword) {
        if (userRepository.existsByEmail(email)) {
            throw new EmailAlreadyInUseException(email);
        }
        userRepository.save(new User(name, email, passwordHasher.hash(rawPassword)));
    }

    @Override
    @Transactional(readOnly = true)
    public User getUserById(UUID id, RequesterContext requester) {
        User target = userRepository.findById(id)
                .orElseThrow(() -> new UserNotFoundException(id));
        if (!requester.isAdmin() && !target.getEmail().equals(requester.email())) {
            throw new AccessDeniedException("You can only access your own account");
        }
        return target;
    }

    @Override
    @Transactional(readOnly = true)
    public User getUserByEmail(String email) {
        return userRepository.findByEmail(email)
                .orElseThrow(() -> new UserNotFoundException(email));
    }

    @Override
    @Transactional(readOnly = true)
    public Page<User> getAllUsers(String search, Pageable pageable) {
        return userRepository.findAll(search, pageable);
    }

    @Override
    @Transactional
    public User updateUser(UUID id, String name, String email, RequesterContext requester) {
        User target = userRepository.findById(id)
                .orElseThrow(() -> new UserNotFoundException(id));
        if (!requester.isAdmin() && !target.getEmail().equals(requester.email())) {
            throw new AccessDeniedException("You can only update your own account");
        }
        return userRepository.save(target.withUpdatedDetails(name, email));
    }

    @Override
    @Transactional
    public User updateUserRole(UUID id, Role role) {
        User existing = userRepository.findById(id)
                .orElseThrow(() -> new UserNotFoundException(id));
        return userRepository.save(existing.withRole(role));
    }

    @Override
    @Transactional
    public void deleteUser(UUID id, RequesterContext requester) {
        User target = userRepository.findById(id)
                .orElseThrow(() -> new UserNotFoundException(id));
        if (!requester.isAdmin() && !target.getEmail().equals(requester.email())) {
            throw new AccessDeniedException("You can only delete your own account");
        }
        userRepository.deleteById(id);
    }
}