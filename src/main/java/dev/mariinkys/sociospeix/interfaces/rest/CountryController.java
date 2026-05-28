package dev.mariinkys.sociospeix.interfaces.rest;

import dev.mariinkys.sociospeix.application.port.CountryUseCase;
import dev.mariinkys.sociospeix.interfaces.dto.country.CountryResponse;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
@RequestMapping("/api/countries")
public class CountryController {

    private final CountryUseCase countryUseCase;

    public CountryController(CountryUseCase countryUseCase) {
        this.countryUseCase = countryUseCase;
    }

    @GetMapping
    public ResponseEntity<List<CountryResponse>> getAll() {
        return ResponseEntity.ok(
                countryUseCase.getAllCountries()
                        .stream()
                        .map(CountryResponse::from)
                        .toList()
        );
    }
}
