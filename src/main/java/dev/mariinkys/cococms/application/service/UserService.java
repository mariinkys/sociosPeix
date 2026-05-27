package dev.mariinkys.cococms.application.service;

import dev.mariinkys.cococms.application.exception.EmailAlreadyInUseException;
import dev.mariinkys.cococms.application.exception.UserNotFoundException;
import dev.mariinkys.cococms.application.port.PasswordHasher;
import dev.mariinkys.cococms.domain.model.User;
import dev.mariinkys.cococms.domain.repository.UserRepository;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Service;

import java.util.UUID;
import dev.mariinkys.cococms.application.port.UserUseCase;

@Service  // Spring registers this as a bean
public class UserService implements UserUseCase {

    private final UserRepository userRepository;
    private final PasswordHasher passwordHasher;

    public UserService(UserRepository userRepository, PasswordHasher passwordHasher) {
        this.userRepository = userRepository;
        this.passwordHasher = passwordHasher;
    }

    @Override
    public User createUser(String name, String email, String rawPassword) {
        if (userRepository.existsByEmail(email)) {
            throw new EmailAlreadyInUseException(email);
        }
        String hashed = passwordHasher.hash(rawPassword);
        return userRepository.save(new User(name, email, hashed));
    }

    @Override
    public User getUserById(UUID id) {
        return userRepository.findById(id)
                .orElseThrow(() -> new UserNotFoundException(id));
    }

    @Override
    public Page<User> getAllUsers(Pageable pageable) {
        return userRepository.findAll(pageable);
    }

    @Override
    public User updateUser(UUID id, String name, String email) {
        User existing = getUserById(id);
        User updated = existing.withUpdatedDetails(name, email);
        return userRepository.save(updated);
    }

    @Override
    public void deleteUser(UUID id) {
        getUserById(id); // throws if not found
        userRepository.deleteById(id);
    }
}
