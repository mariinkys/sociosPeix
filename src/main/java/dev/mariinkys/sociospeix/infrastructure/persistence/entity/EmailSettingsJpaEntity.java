package dev.mariinkys.sociospeix.infrastructure.persistence.entity;

import jakarta.persistence.*;

@Entity
@Table(name = "email_settings")
public class EmailSettingsJpaEntity {

    @Id
    private Integer id;

    @Column(name = "active_provider", nullable = false, length = 50)
    private String activeProvider;

    protected EmailSettingsJpaEntity() {}

    public EmailSettingsJpaEntity(Integer id, String activeProvider) {
        this.id = id;
        this.activeProvider = activeProvider;
    }

    public Integer getId() { return id; }
    public String getActiveProvider() { return activeProvider; }
    public void setActiveProvider(String activeProvider) { this.activeProvider = activeProvider; }
}