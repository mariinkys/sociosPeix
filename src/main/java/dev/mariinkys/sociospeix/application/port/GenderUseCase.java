package dev.mariinkys.sociospeix.application.port;

import dev.mariinkys.sociospeix.domain.model.Gender;
import java.util.List;

public interface GenderUseCase {
    List<Gender> getAllGenders();
}
