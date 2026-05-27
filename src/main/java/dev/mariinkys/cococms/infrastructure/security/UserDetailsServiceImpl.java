package dev.mariinkys.cococms.infrastructure.security;

import dev.mariinkys.cococms.infrastructure.persistence.repository.UserJpaRepository;
import org.jspecify.annotations.NullMarked;
import org.springframework.security.core.userdetails.User;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Service;

@Service
@NullMarked
public class UserDetailsServiceImpl implements UserDetailsService {

    private final UserJpaRepository userJpaRepository;

    public UserDetailsServiceImpl(UserJpaRepository userJpaRepository) {
        this.userJpaRepository = userJpaRepository;
    }

    @Override
    public UserDetails loadUserByUsername(String email) throws UsernameNotFoundException {
        var entity = userJpaRepository.findByEmail(email)
                .orElseThrow(() -> new UsernameNotFoundException("User not found: " + email));

        return User.builder()
                .username(entity.getEmail())
                .password(entity.getPassword())
                .roles(entity.getRole().name())
                .accountLocked(entity.isCurrentlyLocked())
                .build();
    }
}