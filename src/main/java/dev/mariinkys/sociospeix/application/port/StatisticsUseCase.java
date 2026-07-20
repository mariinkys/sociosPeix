package dev.mariinkys.sociospeix.application.port;

import dev.mariinkys.sociospeix.domain.model.statistics.InterestPopularity;

import java.util.List;

public interface StatisticsUseCase {
    List<InterestPopularity> getInterestPopularity();
    byte[] exportInterestPopularityExcel();
}
