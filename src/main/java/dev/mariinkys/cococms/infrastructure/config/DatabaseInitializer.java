package dev.mariinkys.cococms.infrastructure.config;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.boot.flyway.autoconfigure.FlywayMigrationStrategy;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import java.sql.DriverManager;

@Configuration
public class DatabaseInitializer {

    @Value("${spring.datasource.url}")
    private String datasourceUrl;

    @Value("${spring.datasource.username}")
    private String username;

    @Value("${spring.datasource.password}")
    private String password;

    @Bean
    public FlywayMigrationStrategy flywayMigrationStrategy() {
        return flyway -> {
            createDatabaseIfNotExists();
            flyway.migrate();
        };
    }

    private void createDatabaseIfNotExists() {
        String dbName = datasourceUrl.substring(datasourceUrl.lastIndexOf("/") + 1);
        String postgresUrl = datasourceUrl.substring(0, datasourceUrl.lastIndexOf("/")) + "/postgres";

        try (var conn = DriverManager.getConnection(postgresUrl, username, password);
             var stmt = conn.createStatement()) {

            var rs = stmt.executeQuery(
                    "SELECT 1 FROM pg_database WHERE datname = '" + dbName + "'"
            );

            if (!rs.next()) {
                stmt.executeUpdate("CREATE DATABASE \"" + dbName + "\"");
                System.out.println("Database created: " + dbName);
            }

        } catch (Exception e) {
            throw new RuntimeException("Failed to create database", e);
        }
    }
}