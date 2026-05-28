package dev.mariinkys.sociospeix.domain.model;

import java.time.LocalDate;
import java.time.LocalDateTime;
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
    }

    // For reconstructing from DB
    public Member(UUID id, String name, String surname, String secondSurname,
                  String email, LocalDate birthdate, String phone, String notes,
                  Gender gender, Country country,
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
    }

    public String getFullName() {
        return (name + " " + surname + " " + secondSurname).trim();
    }

    public Member withUpdatedDetails(String name, String surname, String secondSurname,
                                     String email, LocalDate birthdate, String phone,
                                     String notes, Gender gender, Country country) {
        return new Member(this.id, name, surname, secondSurname, email, birthdate,
                phone, notes, gender, country, this.createdAt, LocalDateTime.now());
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
}