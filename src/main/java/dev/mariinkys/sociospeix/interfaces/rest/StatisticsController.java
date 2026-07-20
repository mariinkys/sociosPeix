package dev.mariinkys.sociospeix.interfaces.rest;

import dev.mariinkys.sociospeix.application.port.StatisticsUseCase;
import dev.mariinkys.sociospeix.interfaces.dto.statistics.InterestPopularityResponse;
import org.springframework.http.HttpHeaders;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
@RequestMapping("/api/statistics")
@PreAuthorize("hasRole('ADMIN')")
public class StatisticsController {

    private final StatisticsUseCase statisticsUseCase;

    public StatisticsController(StatisticsUseCase statisticsUseCase) {
        this.statisticsUseCase = statisticsUseCase;
    }

    @GetMapping("/interests/popularity")
    public ResponseEntity<List<InterestPopularityResponse>> interestPopularity() {
        var response = statisticsUseCase.getInterestPopularity().stream()
                .map(InterestPopularityResponse::from)
                .toList();
        return ResponseEntity.ok(response);
    }

    @GetMapping("/interests/popularity/export")
    public ResponseEntity<byte[]> exportInterestPopularity() {
        byte[] file = statisticsUseCase.exportInterestPopularityExcel();
        return ResponseEntity.ok()
                .header(HttpHeaders.CONTENT_DISPOSITION, "attachment; filename=\"interest-popularity.xlsx\"")
                .contentType(MediaType.parseMediaType("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"))
                .body(file);
    }
}