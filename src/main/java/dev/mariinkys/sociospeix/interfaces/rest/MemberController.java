package dev.mariinkys.sociospeix.interfaces.rest;

import dev.mariinkys.sociospeix.application.port.MemberUseCase;
import dev.mariinkys.sociospeix.interfaces.dto.member.MemberRequest;
import dev.mariinkys.sociospeix.interfaces.dto.member.MemberResponse;
import dev.mariinkys.sociospeix.interfaces.dto.PageResponse;
import jakarta.validation.Valid;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.UUID;

@RestController
@RequestMapping("/api/members")
public class MemberController {

    private final MemberUseCase memberUseCase;

    public MemberController(MemberUseCase memberUseCase) {
        this.memberUseCase = memberUseCase;
    }

    @GetMapping
    public ResponseEntity<PageResponse<MemberResponse>> getAll(
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size,
            @RequestParam(defaultValue = "surname") String sortBy,
            @RequestParam(defaultValue = "asc") String sortDir,
            @RequestParam(defaultValue = "") String search,
            @RequestParam(required = false) List<Integer> interestIds) {

        var direction = sortDir.equalsIgnoreCase("asc") ? Sort.Direction.ASC : Sort.Direction.DESC;
        var pageable = PageRequest.of(page, size, Sort.by(direction, sortBy));
        var result = memberUseCase.getAllMembers(search, interestIds, pageable)
                .map(MemberResponse::from);
        return ResponseEntity.ok(PageResponse.from(result));
    }

    @GetMapping("/export")
    public ResponseEntity<byte[]> export(
            @RequestParam(defaultValue = "") String search,
            @RequestParam(required = false) List<Integer> interestIds) {

        byte[] file = memberUseCase.exportMembers(search, interestIds);

        return ResponseEntity.ok()
                .header(HttpHeaders.CONTENT_DISPOSITION, "attachment; filename=\"members.xlsx\"")
                .contentType(MediaType.parseMediaType("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"))
                .body(file);
    }

    @GetMapping("/{id}")
    public ResponseEntity<MemberResponse> getById(@PathVariable UUID id) {
        return ResponseEntity.ok(MemberResponse.from(memberUseCase.getMemberById(id)));
    }

    @GetMapping("/birthdays/today")
    public ResponseEntity<List<MemberResponse>> getTodaysBirthdays() {
        var members = memberUseCase.getTodaysBirthdays()
                .stream()
                .map(MemberResponse::from)
                .toList();
        return ResponseEntity.ok(members);
    }

    @PostMapping
    public ResponseEntity<MemberResponse> create(@Valid @RequestBody MemberRequest request) {
        var member = memberUseCase.createMember(
                request.name(), request.surname(), request.secondSurname(),
                request.email(), request.birthdate(), request.phone(), request.notes(),
                request.genderId(), request.countryId(), request.interestIds()
        );
        return ResponseEntity.status(HttpStatus.CREATED).body(MemberResponse.from(member));
    }

    @PutMapping("/{id}")
    public ResponseEntity<MemberResponse> update(@PathVariable UUID id,
                                                 @Valid @RequestBody MemberRequest request) {
        var member = memberUseCase.updateMember(
                id, request.name(), request.surname(), request.secondSurname(),
                request.email(), request.birthdate(), request.phone(), request.notes(),
                request.genderId(), request.countryId(), request.interestIds()
        );
        return ResponseEntity.ok(MemberResponse.from(member));
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Void> delete(@PathVariable UUID id) {
        memberUseCase.deleteMember(id);
        return ResponseEntity.noContent().build();
    }
}