package dev.mariinkys.sociospeix.domain.model;

import java.time.LocalDate;
import java.time.LocalDateTime;
import java.util.UUID;

public class Member {

    private final UUID id;
    private final String name;
    private final String surname;
    private final String secondSurname;
    private final String email;
    private final LocalDate birthdate;
    private final String phone;
    private final String notes;
    private final LocalDateTime createdAt;
    private final LocalDateTime updatedAt;

    // For creating a new member
    public Member(String name, String surname, String secondSurname,
                  String email, LocalDate birthdate, String phone, String notes) {
        this.id = null;
        this.name = name;
        this.surname = surname;
        this.secondSurname = secondSurname;
        this.email = email;
        this.birthdate = birthdate;
        this.phone = phone;
        this.notes = notes;
        this.createdAt = null;
        this.updatedAt = null;
    }

    // For reconstructing from DB
    public Member(UUID id, String name, String surname, String secondSurname,
                  String email, LocalDate birthdate, String phone, String notes,
                  LocalDateTime createdAt, LocalDateTime updatedAt) {
        this.id = id;
        this.name = name;
        this.surname = surname;
        this.secondSurname = secondSurname;
        this.email = email;
        this.birthdate = birthdate;
        this.phone = phone;
        this.notes = notes;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    public String getFullName() {
        return (name + " " + surname + " " + secondSurname).trim();
    }

    public Member withUpdatedDetails(String name, String surname, String secondSurname,
                                     String email, LocalDate birthdate,
                                     String phone, String notes) {
        return new Member(this.id, name, surname, secondSurname, email,
                birthdate, phone, notes, this.createdAt, LocalDateTime.now());
    }

    public UUID getId() { return id; }
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