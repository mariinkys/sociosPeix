package dev.mariinkys.sociospeix.interfaces.rest;

import dev.mariinkys.sociospeix.application.port.InterestUseCase;
import dev.mariinkys.sociospeix.interfaces.dto.interest.InterestRequest;
import dev.mariinkys.sociospeix.interfaces.dto.interest.InterestResponse;
import jakarta.validation.Valid;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/interests")
public class InterestController {

    private final InterestUseCase interestUseCase;

    public InterestController(InterestUseCase interestUseCase) {
        this.interestUseCase = interestUseCase;
    }

    @GetMapping
    public ResponseEntity<List<InterestResponse>> getAll() {
        return ResponseEntity.ok(
                interestUseCase.getAllInterests().stream().map(InterestResponse::from).toList()
        );
    }

    @GetMapping("/{id}")
    public ResponseEntity<InterestResponse> getById(@PathVariable Integer id) {
        return ResponseEntity.ok(InterestResponse.from(interestUseCase.getInterestById(id)));
    }

    @PostMapping
    public ResponseEntity<InterestResponse> create(@Valid @RequestBody InterestRequest request) {
        return ResponseEntity.status(HttpStatus.CREATED).body(
                InterestResponse.from(interestUseCase.createInterest(request.name(), request.description()))
        );
    }

    @PutMapping("/{id}")
    public ResponseEntity<InterestResponse> update(@PathVariable Integer id,
                                                   @Valid @RequestBody InterestRequest request) {
        return ResponseEntity.ok(
                InterestResponse.from(interestUseCase.updateInterest(id, request.name(), request.description()))
        );
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Void> delete(@PathVariable Integer id) {
        interestUseCase.deleteInterest(id);
        return ResponseEntity.noContent().build();
    }
}
