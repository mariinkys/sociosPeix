package dev.mariinkys.sociospeix.application.port;

import dev.mariinkys.sociospeix.domain.model.Interest;

import java.util.List;

public interface InterestUseCase {
    Interest createInterest(String name, String description);
    Interest getInterestById(Integer id);
    List<Interest> getAllInterests();
    Interest updateInterest(Integer id, String name, String description);
    void deleteInterest(Integer id);
}
