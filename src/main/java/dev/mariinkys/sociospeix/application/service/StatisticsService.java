package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.application.port.ExcelExporter;
import dev.mariinkys.sociospeix.application.port.StatisticsUseCase;
import dev.mariinkys.sociospeix.domain.model.statistics.InterestPopularity;
import dev.mariinkys.sociospeix.domain.repository.StatisticsRepository;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;

@Service
public class StatisticsService implements StatisticsUseCase {

    private final StatisticsRepository statisticsRepository;
    private final ExcelExporter<InterestPopularity> interestPopularityExcelExporter;

    public StatisticsService(StatisticsRepository statisticsRepository,
                             ExcelExporter<InterestPopularity> interestPopularityExcelExporter) {
        this.statisticsRepository = statisticsRepository;
        this.interestPopularityExcelExporter = interestPopularityExcelExporter;
    }

    @Override
    @Transactional(readOnly = true)
    public List<InterestPopularity> getInterestPopularity() {
        return statisticsRepository.countMembersPerInterest();
    }

    @Override
    @Transactional(readOnly = true)
    public byte[] exportInterestPopularityExcel() {
        return interestPopularityExcelExporter.export(getInterestPopularity());
    }
}