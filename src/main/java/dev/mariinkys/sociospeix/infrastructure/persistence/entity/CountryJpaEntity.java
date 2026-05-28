package dev.mariinkys.sociospeix.infrastructure.persistence.entity;

import jakarta.persistence.*;

@Entity
@Table(name = "countries")
public class CountryJpaEntity {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id;

    @Column(nullable = false, unique = true, length = 100)
    private String name;

    protected CountryJpaEntity() {}

    public Integer getId() { return id; }
    public String getName() { return name; }
}