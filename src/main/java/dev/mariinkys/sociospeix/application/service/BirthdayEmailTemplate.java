package dev.mariinkys.sociospeix.application.service;

import dev.mariinkys.sociospeix.domain.model.EmailAttachment;
import org.springframework.core.io.ClassPathResource;
import org.springframework.stereotype.Component;

import java.io.IOException;

@Component
public class BirthdayEmailTemplate {

    private final byte[] logoBytes;

    public BirthdayEmailTemplate() throws IOException {
        this.logoBytes = new ClassPathResource("static/logo.png").getInputStream().readAllBytes();
    }

    public String build(String memberName) {
        return """
            <!DOCTYPE html>
            <html>
            <body style="font-family: Arial, sans-serif; background-color: #fff7e6; color: #333;">
              <div style="max-width: 600px; margin: auto; background: #fff; border-radius: 8px;
                          padding: 30px; box-shadow: 0 2px 6px rgba(0,0,0,0.1); text-align: center;">
                <img src="cid:logo" alt="Hotel Casa Peix (Logo)" style="max-width: 200px; margin-bottom: 20px;"/>
                <h1 style="color: #d35400;">¡Feliz Cumpleaños, %s!</h1>
                <p style="font-size: 16px; line-height: 1.6;">
                  Esperamos que este día esté lleno de alegría, momentos especiales y felicidad.
                </p>
                <p style="font-size: 16px; line-height: 1.6;">
                  Como muestra de nuestro cariño, queremos recordarle que siempre será
                  bienvenido(a) en nuestro hotel.
                </p>
                <p style="font-size: 14px; color: #5b5b5b; margin-top: 30px;">
                  Con nuestros mejores deseos,<br/>El equipo del Hotel Casa Peix.
                </p>
                <p style="font-size: 12px; color: #777; text-align: center;">
                  (Puede darse de baja en cualquier momento escribiendo un correo a
                  infohotelcasapeix@gmail.com)
                </p>
              </div>
            </body>
            </html>
            """.formatted(memberName);
    }

    public EmailAttachment logoAttachment() {
        return new EmailAttachment("logo.png", "image/png", logoBytes, "logo");
    }
}