import logoBase64 from '@/assets/logo.png?inline'

export type EmailTemplate = 'none' | 'basic' | 'birthday'

const LOGO_IMG = `<div style="text-align: center; margin-bottom: 20px;">
  <img src="${logoBase64}" alt="Hotel Casa Peix" style="width: 250px; height: auto;" />
</div>`

export function applyTemplate(body: string, template: EmailTemplate): string {
  const inlinedBody = quillToInlineStyles(body)
  if (template === 'none') return inlinedBody

  if (template === 'basic') {
    return `<!DOCTYPE html>
      <html>
      <body style="font-family: Arial, sans-serif; background-color: #f9f9f9; color: #333;">
        <div style="max-width: 600px; margin: auto; background: #fff; border-radius: 8px; padding: 20px; box-shadow: 0 2px 6px rgba(0,0,0,0.1);">
          ${LOGO_IMG}
          <p style="font-size: 16px; line-height: 1.6;">${inlinedBody}</p>
          <p style="font-size: 14px; color: #5b5b5b; text-align: center; margin-top: 30px;">
            ¡Gracias por confiar en nosotros!<br/>El equipo del Hotel Casa Peix.
          </p>
          <p style="font-size: 12px; color: #777; text-align: center;">
            (Puede darse de baja en cualquier momento escribiendo un correo a infohotelcasapeix@gmail.com)
          </p>
        </div>
      </body>
      </html>`
  }

  if (template === 'birthday') {
    return `<!DOCTYPE html>
      <html>
      <body style="font-family: Arial, sans-serif; background-color: #fff7e6; color: #333;">
        <div style="max-width: 600px; margin: auto; background: #fff; border-radius: 8px; padding: 30px; box-shadow: 0 2px 6px rgba(0,0,0,0.1); text-align: center;">
          ${LOGO_IMG}
          <h1 style="color: #d35400;">¡Feliz Cumpleaños!</h1>
          <p style="font-size: 16px; line-height: 1.6;">
            Esperamos que este día esté lleno de alegría, momentos especiales y felicidad.
          </p>
          <p style="font-size: 16px; line-height: 1.6;">
            Como muestra de nuestro cariño, queremos recordarle que siempre será bienvenido(a) en nuestro hotel.
          </p>
          <p style="font-size: 14px; color: #5b5b5b; margin-top: 30px;">
            Con nuestros mejores deseos,<br/>El equipo del Hotel Casa Peix.
          </p>
          <p style="font-size: 12px; color: #777; text-align: center;">
            (Puede darse de baja en cualquier momento escribiendo un correo a infohotelcasapeix@gmail.com)
          </p>
        </div>
      </body>
      </html>`
  }

  return body
}

function quillToInlineStyles(html: string): string {
  return html
    .replace(/class="ql-align-center"/g, 'style="text-align: center;"')
    .replace(/class="ql-align-right"/g, 'style="text-align: right;"')
    .replace(/class="ql-align-justify"/g, 'style="text-align: justify;"')
    .replace(/class="ql-indent-1"/g, 'style="padding-left: 3em;"')
    .replace(/class="ql-indent-2"/g, 'style="padding-left: 6em;"')
    .replace(/class="ql-indent-3"/g, 'style="padding-left: 9em;"')
}

export function wrapEmailBody(body: string): string {
  return `<!DOCTYPE html>
    <html>
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <style>
          body {
            margin: 12px;
            font-family: sans-serif;
            font-size: 14px;
            line-height: 1.5;
            color: #1a1a1a;
            word-break: break-word;
          }
          img { max-width: 100%; height: auto; }
        </style>
      </head>
      <body>${body}</body>
    </html>`
}

export const TEMPLATE_OPTIONS = [
  { value: 'none' as const, label: 'No template', icon: 'pi-align-left' },
  { value: 'basic' as const, label: 'Basic', icon: 'pi-envelope' },
  // { value: 'birthday' as const, label: 'Birthday', icon: 'pi-gift' },
]
