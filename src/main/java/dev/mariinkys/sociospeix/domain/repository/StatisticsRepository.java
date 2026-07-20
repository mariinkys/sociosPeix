package dev.mariinkys.sociospeix.domain.repository;

import dev.mariinkys.sociospeix.domain.model.statistics.InterestPopularity;

import java.util.List;

public interface StatisticsRepository {
    List<InterestPopularity> countMembersPerInterest();
}