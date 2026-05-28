package dev.mariinkys.sociospeix.domain.repository;

import dev.mariinkys.sociospeix.domain.model.Gender;
import java.util.List;
import java.util.Optional;

public interface GenderRepository {
    List<Gender> findAll();
    Optional<Gender> findById(Integer id);
}
