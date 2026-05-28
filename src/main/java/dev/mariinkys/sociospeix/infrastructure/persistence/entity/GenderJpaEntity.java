package dev.mariinkys.sociospeix.infrastructure.persistence.entity;

import jakarta.persistence.*;

@Entity
@Table(name = "genders")
public class GenderJpaEntity {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id;

    @Column(nullable = false, unique = true, length = 50)
    private String name;

    protected GenderJpaEntity() {}

    public Integer getId() { return id; }
    public String getName() { return name; }
}