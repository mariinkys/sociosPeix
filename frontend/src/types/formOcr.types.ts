export interface FormOcrField {
  text: string
  lines: string[]
  confidence: number | null
  empty: boolean
}

export interface FormOcrPage {
  page: number
  fields: {
    NOMBRE: FormOcrField
    APELLIDO: FormOcrField
    SEGUNDO_APELLIDO: FormOcrField
    FECHA_NACIMIENTO: FormOcrField
    GENERO: FormOcrField
    TELEFONO: FormOcrField
    CORREO_ELECTRONICO: FormOcrField
    PAIS: FormOcrField
  }
}

export interface FormOcrResponse {
  pages: FormOcrPage[]
}
