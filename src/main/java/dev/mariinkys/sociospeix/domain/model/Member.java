package dev.mariinkys.sociospeix.domain.model;

import java.time.LocalDate;
import java.time.LocalDateTime;
import java.util.List;
import java.util.UUID;

public class Member {

    private UUID id;
    private final String name;
    private final String surname;
    private final String secondSurname;
    private final String email;
    private final LocalDate birthdate;
    private final String phone;
    private final String notes;
    private final Gender gender;
    private final Country country;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;

    private final List<Interest> interests;

    // For creating a new member
    public Member(String name, String surname, String secondSurname,
                  String email, LocalDate birthdate, String phone, String notes, Gender gender, Country country) {
        this.name = name;
        this.gender = gender;
        this.country = country;
        this.surname = surname;
        this.secondSurname = secondSurname;
        this.email = email;
        this.birthdate = birthdate;
        this.phone = phone;
        this.notes = notes;

        this.interests = List.of();
    }

    // For reconstructing from DB
    public Member(UUID id, String name, String surname, String secondSurname,
                  String email, LocalDate birthdate, String phone, String notes,
                  Gender gender, Country country, List<Interest> interests,
                  LocalDateTime createdAt, LocalDateTime updatedAt) {
        this.id = id;
        this.name = name;
        this.surname = surname;
        this.secondSurname = secondSurname;
        this.email = email;
        this.birthdate = birthdate;
        this.phone = phone;
        this.notes = notes;
        this.gender = gender;
        this.country = country;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;

        this.interests = interests != null ? interests : List.of();
    }

    public String getFullName() {
        return (name + " " + surname + " " + secondSurname).trim();
    }

    public Member withUpdatedDetails(String name, String surname, String secondSurname,
                                     String email, LocalDate birthdate, String phone,
                                     String notes, Gender gender, Country country) {
        return new Member(this.id, name, surname, secondSurname, email, birthdate,
                phone, notes, gender, country, this.interests, this.createdAt, LocalDateTime.now());
    }

    public Member withInterests(List<Interest> interests) {
        return new Member(this.id, this.name, this.surname, this.secondSurname,
                this.email, this.birthdate, this.phone, this.notes,
                this.gender, this.country, interests,
                this.createdAt, this.updatedAt);
    }

    public UUID getId() { return id; }
    public Gender getGender() { return gender; }
    public Country getCountry() { return country; }
    public String getName() { return name; }
    public String getSurname() { return surname; }
    public String getSecondSurname() { return secondSurname; }
    public String getEmail() { return email; }
    public LocalDate getBirthdate() { return birthdate; }
    public String getPhone() { return phone; }
    public String getNotes() { return notes; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public LocalDateTime getUpdatedAt() { return updatedAt; }

    public List<Interest> getInterests() { return interests; }
}