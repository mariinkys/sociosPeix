package dev.mariinkys.sociospeix.interfaces.dto.member;

import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;

import java.util.List;

public record MemberRequest(
        @NotBlank(message = "Name is required")
        @Size(max = 100, message = "Name must not exceed 100 characters")
        String name,

        @NotBlank(message = "Surname is required")
        @Size(max = 100, message = "Surname must not exceed 100 characters")
        String surname,

        @Size(max = 100, message = "Second surname must not exceed 100 characters")
        String secondSurname,

        @NotBlank(message = "Email is required")
        @Email(message = "Email must be valid")
        String email,

        String birthdate,

        @Size(max = 30, message = "Phone must not exceed 30 characters")
        String phone,

        String notes,

        Integer genderId,
        Integer countryId,

        List<Integer> interestIds
) {}