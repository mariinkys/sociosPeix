package dev.mariinkys.sociospeix.infrastructure.persistence.entity;

import jakarta.persistence.*;
import java.time.LocalDate;
import java.time.LocalDateTime;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

@Entity
@Table(name = "members")
public class MemberJpaEntity {

    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    private UUID id;

    @ManyToOne(fetch = FetchType.EAGER)
    @JoinColumn(name = "gender_id")
    private GenderJpaEntity gender;

    @ManyToOne(fetch = FetchType.EAGER)
    @JoinColumn(name = "country_id")
    private CountryJpaEntity country;

    @Column(nullable = false, length = 100)
    private String name;

    @Column(nullable = false, length = 100)
    private String surname;

    @Column(name = "second_surname", nullable = false, length = 100)
    private String secondSurname;

    @Column(nullable = false, length = 255)
    private String email;

    @Column
    private LocalDate birthdate;

    @Column(nullable = false, length = 30)
    private String phone;

    @Column(nullable = false, columnDefinition = "TEXT")
    private String notes;

    @Column(name = "created_at", nullable = false, updatable = false)
    private LocalDateTime createdAt;

    @Column(name = "updated_at", nullable = false)
    private LocalDateTime updatedAt;

    @ManyToMany(fetch = FetchType.LAZY)
    @JoinTable(
            name = "member_interests",
            joinColumns = @JoinColumn(name = "member_id"),
            inverseJoinColumns = @JoinColumn(name = "interest_id")
    )
    private List<InterestJpaEntity> interests = new ArrayList<>();

    @PrePersist
    protected void onCreate() {
        createdAt = LocalDateTime.now();
        updatedAt = LocalDateTime.now();
    }

    @PreUpdate
    protected void onUpdate() {
        updatedAt = LocalDateTime.now();
    }

    protected MemberJpaEntity() {}

    public MemberJpaEntity(UUID id, String name, String surname, String secondSurname,
                           String email, LocalDate birthdate, String phone, String notes,
                           GenderJpaEntity gender, CountryJpaEntity country,
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

    public UUID getId() { return id; }
    public String getName() { return name; }
    public String getSurname() { return surname; }
    public String getSecondSurname() { return secondSurname; }
    public String getEmail() { return email; }
    public LocalDate getBirthdate() { return birthdate; }
    public String getPhone() { return phone; }
    public String getNotes() { return notes; }
    public GenderJpaEntity getGender() { return gender; }
    public CountryJpaEntity getCountry() { return country; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public LocalDateTime getUpdatedAt() { return updatedAt; }

    public List<InterestJpaEntity> getInterests() { return interests; }
    public void setInterests(List<InterestJpaEntity> interests) { this.interests = interests; }
}