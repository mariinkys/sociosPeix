package dev.mariinkys.sociospeix.interfaces.rest;

import dev.mariinkys.sociospeix.application.port.GenderUseCase;
import dev.mariinkys.sociospeix.interfaces.dto.gender.GenderResponse;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
@RequestMapping("/api/genders")
public class GenderController {

    private final GenderUseCase genderUseCase;

    public GenderController(GenderUseCase genderUseCase) {
        this.genderUseCase = genderUseCase;
    }

    @GetMapping
    public ResponseEntity<List<GenderResponse>> getAll() {
        return ResponseEntity.ok(
                genderUseCase.getAllGenders()
                        .stream()
                        .map(GenderResponse::from)
                        .toList()
        );
    }
}