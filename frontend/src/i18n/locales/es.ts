export default {
  common: {
    actions: {
      cancel: 'Cancelar',
      close: 'Cerrar',
      apply: 'Aplicar',
      saveChanges: 'Guardar cambios',
      refresh: 'Actualizar',
      back: 'Volver',
      create: 'Crear',
      delete: 'Eliminar',
      export: 'Exportar',
      send: 'Enviar',
      preview: 'Vista previa',
      addFiles: 'Añadir archivos',
      removeAttachment: 'Quitar archivo adjunto',
      composeEmail: 'Redactar correo',
      signOut: 'Cerrar sesión',
      help: 'Ayuda',
    },

    feedback: {
      error: 'Error',
      saved: 'Guardado',
      created: 'Creado',
      deleted: 'Eliminado',
      sent: 'Enviado',
    },

    fields: {
      name: 'Nombre',
      surname: 'Apellido',
      secondSurname: 'Segundo Apellido',
      email: 'Correo Electrónico',
      password: 'Contraseña',
      currentPassword: 'Contraseña Actual',
      newPassword: 'Contraseña Nueva',
      confirmPassword: 'Confirmar Contraseña Nueva',
      description: 'Descripción',
      phone: 'Teléfono',
      birthdate: 'Fecha de Nacimiento',
      gender: 'Género',
      country: 'País',
      notes: 'Notas',
      interests: 'Intereses',
      role: 'Rol',
      subject: 'Asunto',
      body: 'Contenido',
      provider: 'Proveedor',
      recipients: 'Destinatarios',
      template: 'Plantilla',
      attachments: 'Archivos Adjuntos',
      member: 'Miembro',
      birthday: 'Cumpleaños',
      sentAt: 'Enviado el',
      sentTo: 'Enviado a',
      createdAt: 'Creado el',
    },

    placeholders: {
      enterName: 'Introduce el nombre',
      enterSurname: 'Introduce el apellido',
      enterSecondSurname: 'Introduce el segundo apellido',
      enterEmail: 'Introduce el correo electrónico',
      enterYourEmail: 'Introduce tu correo electrónico',
      enterPassword: 'Introduce la contraseña',
      enterCurrentPassword: 'Introduce la contraseña actual',
      enterNewPassword: 'Introduce la nueva contraseña',
      repeatNewPassword: 'Repite la nueva contraseña',
      enterPhone: 'Introduce el teléfono',
      enterDescription: 'Introduce una descripción',
      enterInterestName: 'Introduce el nombre del interés',
      enterEmailSubject: 'Introduce el asunto del correo',
      additionalNotes: 'Cualquier nota adicional',
      search: 'Buscar...',
      searchUsers: 'Buscar usuarios...',
      searchMembers: 'Buscar miembros...',
      searchInterests: 'Buscar intereses...',
      searchGender: 'Buscar género...',
      searchCountry: 'Buscar país...',
      filterByInterest: 'Filtrar por interés...',
      selectInterests: 'Selecciona intereses...',
    },

    validation: {
      required: 'Este campo es obligatorio',
      emailRequired: 'El correo electrónico es obligatorio',
      passwordRequired: 'La contraseña es obligatoria',
      invalidEmail: 'Debe ser un correo electrónico válido',
      emailInvalid: 'El correo electrónico debe ser válido',
      nameRequired: 'El nombre es obligatorio',
      surnameRequired: 'El apellido es obligatorio',
      currentPasswordRequired: 'La contraseña actual es obligatoria',
      newPasswordRequired: 'La nueva contraseña es obligatoria',
      confirmNewPasswordRequired: 'Por favor, confirma la nueva contraseña',
      interestsRequired: 'Selecciona al menos un interés',
      passwordsMismatch: 'Las contraseñas no coinciden',
      max100: 'No debe superar los 100 caracteres',
      min8: 'Debe tener al menos 8 caracteres',
    },

    states: {
      emptyValue: '—',
      noResults: 'No se han encontrado resultados',
      noEmailsSentYet: 'Todavía no se ha enviado ningún correo',
      resetsAtMidnight: 'Se restablece a medianoche',
    },

    theme: {
      switchToLightMode: 'Cambiar a modo claro',
      switchToDarkMode: 'Cambiar a modo oscuro',
      toggleMenu: 'Mostrar u ocultar menú',
    },
  },

  home: {
    title: 'Panel',
    description: 'Bienvenido de Nuevo',
  },

  tools: {
    title: 'Herramientas',
    description: 'Varias Herramientas y Utilidades',
  },

  navigation: {
    items: {
      users: 'Usuarios',
      members: 'Miembros',
      interests: 'Intereses',
      tools: 'Herramientas',
    },
  },

  auth: {
    login: {
      title: 'Iniciar Sesión',
      description: 'Bienvenido de nuevo — Introduce tus Datos',
      submit: 'Iniciar Sesión',
      errors: {
        loginFailed: 'Error al iniciar sesión',
      },
    },
  },

  users: {
    roles: {
      USER: 'Usuario',
      ADMIN: 'Administrador',
    },

    titles: {
      list: 'Usuarios',
      create: 'Nuevo Usuario',
      edit: 'Editar Usuario',
    },

    descriptions: {
      create: 'Completa los datos para crear un nuevo usuario',
      edit: 'Actualiza los datos del usuario a continuación',
    },

    actions: {
      create: 'Crear Usuario',
      createNew: 'Nuevo Usuario',
      delete: 'Eliminar Usuario',
    },

    list: {
      total: '{total} usuarios en total',
      empty: 'No se han encontrado usuarios',
    },

    fields: {
      name: {
        label: '@:common.fields.name',
        placeholder: '@:common.placeholders.enterName',
        required: '@:common.validation.nameRequired',
        max: 'El nombre no debe superar los 100 caracteres',
      },
      email: {
        label: '@:common.fields.email',
        placeholder: '@:common.placeholders.enterEmail',
        required: '@:common.validation.emailRequired',
        invalid: '@:common.validation.emailInvalid',
      },
      password: {
        label: '@:common.fields.password',
        placeholder: '@:common.placeholders.enterPassword',
        required: '@:common.validation.passwordRequired',
        min: 'La contraseña debe tener al menos 8 caracteres',
      },
    },

    passwordCard: {
      title: 'Cambiar Contraseña',
      submit: 'Actualizar Contraseña',
      fields: {
        currentPassword: {
          label: '@:common.fields.currentPassword',
          placeholder: '@:common.placeholders.enterCurrentPassword',
          required: '@:common.validation.currentPasswordRequired',
        },
        newPassword: {
          label: '@:common.fields.newPassword',
          placeholder: '@:common.placeholders.enterNewPassword',
          required: '@:common.validation.newPasswordRequired',
          min: 'La contraseña debe tener al menos 8 caracteres',
        },
        confirmPassword: {
          label: '@:common.fields.confirmPassword',
          placeholder: '@:common.placeholders.repeatNewPassword',
          required: '@:common.validation.confirmNewPasswordRequired',
          mismatch: '@:common.validation.passwordsMismatch',
        },
      },
      successTitle: 'Contraseña Actualizada',
      successDetail: 'La contraseña se cambió correctamente.',
      error:
        'No se pudo actualizar la contraseña. Comprueba tu contraseña actual e inténtalo de nuevo.',
    },

    roleCard: {
      title: '@:common.fields.role',
      description: 'Cambiar esto afectará inmediatamente a lo que el usuario puede acceder.',
      updatedTitle: 'Rol Actualizado',
      updatedDetail: 'Rol cambiado a {role}',
      updateError: 'No se pudo actualizar el rol',
    },

    deleteDialog: {
      title: 'Eliminar Usuario',
      message: '¿Seguro que quieres eliminar este usuario? Esta acción no se puede deshacer.',
      success: 'Usuario eliminado correctamente.',
      error: 'No se pudo eliminar el usuario. Inténtalo de nuevo.',
    },

    messages: {
      updated: 'Usuario actualizado correctamente',
      created: 'Usuario creado correctamente',
      updateError: 'Fallo al actualizar el usuario',
      createError: 'Fallo al crear el usuario',
      loadError: 'No se pudo cargar el usuario',
      loadListError: 'No se pudieron cargar los usuarios. Inténtalo de nuevo.',
    },
  },

  members: {
    titles: {
      list: 'Miembros',
      create: 'Nuevo Miembro',
      edit: 'Editar Miembro',
    },

    descriptions: {
      create: 'Completa los datos para crear un nuevo miembro',
      edit: 'Actualiza los datos del miembro a continuación',
    },

    actions: {
      create: 'Crear Miembro',
      createNew: 'Nuevo Miembro',
      delete: 'Eliminar Miembro',
    },

    sections: {
      personalDetails: 'Datos Personales',
      contact: 'Contacto',
      notes: 'Notas',
      interests: 'Intereses',
    },

    list: {
      total: '{total} miembros en total',
      empty: 'No se han encontrado miembros',
      exportError: 'No se pudieron exportar los miembros.',
    },

    filters: {
      searchPlaceholder: '@:common.placeholders.searchMembers',
      interestsPlaceholder: '@:common.placeholders.filterByInterest',
      interestsSearchPlaceholder: '@:common.placeholders.search',
    },

    fields: {
      name: {
        label: '@:common.fields.name',
        placeholder: '@:common.placeholders.enterName',
        required: '@:common.validation.nameRequired',
        max: 'El nombre no debe superar los 100 caracteres',
      },
      surname: {
        label: '@:common.fields.surname',
        placeholder: '@:common.placeholders.enterSurname',
        required: '@:common.validation.surnameRequired',
        max: 'El apellido no debe superar los 100 caracteres',
      },
      secondSurname: {
        label: '@:common.fields.secondSurname',
        placeholder: '@:common.placeholders.enterSecondSurname',
        max: 'El segundo apellido no debe superar los 100 caracteres',
      },
      email: {
        label: '@:common.fields.email',
        placeholder: '@:common.placeholders.enterEmail',
        required: '@:common.validation.emailRequired',
        invalid: '@:common.validation.emailInvalid',
        editWarning:
          'Cambiar el correo electrónico ocultará los correos enviados anteriormente, ya que se asocian por dirección de correo.',
      },
      birthdate: {
        label: '@:common.fields.birthdate',
      },
      gender: {
        label: '@:common.fields.gender',
      },
      phone: {
        label: '@:common.fields.phone',
        placeholder: '@:common.placeholders.enterPhone',
        max: 'El teléfono no debe superar los 30 caracteres',
      },
      country: {
        label: '@:common.fields.country',
      },
      notes: {
        label: '@:common.fields.notes',
        placeholder: '@:common.placeholders.additionalNotes',
      },
      interests: {
        label: '@:common.fields.interests',
        help: 'Selecciona uno o más intereses para este miembro',
      },
    },

    todayBirthdayCard: {
      title: 'Cumpleaños de Hoy',
      empty: 'No hay cumpleaños hoy',
      errors: {
        load: 'No se pudieron cargar los cumpleaños de hoy.',
      },
    },

    deleteDialog: {
      title: 'Eliminar Miembro',
      message: '¿Seguro que quieres eliminar a {name}? Esta acción no se puede deshacer.',
      messageGeneric:
        '¿Seguro que quieres eliminar este miembro? Esta acción no se puede deshacer.',
      success: '{name} ha sido eliminado.',
      deletedSuccess: 'Miembro eliminado correctamente.',
      error: 'No se pudo eliminar el miembro. Inténtalo de nuevo.',
    },

    messages: {
      updated: 'Miembro actualizado correctamente',
      created: 'Miembro creado correctamente',
      updateError: 'Fallo al actualizar el miembro',
      createError: 'Fallo al crear el miembro',
      loadError: 'No se pudo cargar el miembro',
      loadListError: 'No se pudieron cargar los miembros. Inténtalo de nuevo.',
    },
  },

  interests: {
    titles: {
      list: 'Intereses',
      create: 'Nuevo Interés',
      edit: 'Editar Interés',
    },

    descriptions: {
      create: 'Completa los datos para crear un nuevo interés',
      edit: 'Actualiza los datos del interés a continuación',
    },

    actions: {
      create: 'Crear Interés',
      createNew: 'Nuevo Interés',
      delete: 'Eliminar Interés',
    },

    list: {
      count: 'Ningún interés | {count} interés | {count} intereses',
      empty: 'No se han encontrado intereses',
    },

    fields: {
      name: {
        label: '@:common.fields.name',
        placeholder: '@:common.placeholders.enterInterestName',
        required: '@:common.validation.nameRequired',
        max: 'El nombre no debe superar los 100 caracteres',
      },
      description: {
        label: '@:common.fields.description',
        placeholder: '@:common.placeholders.enterDescription',
      },
    },

    multiSelect: {
      placeholder: '@:common.placeholders.selectInterests',
      filterPlaceholder: '@:common.placeholders.searchInterests',
    },

    deleteDialog: {
      title: 'Eliminar Interés',
      message: '¿Seguro que quieres eliminar "{name}"? Esta acción no se puede deshacer.',
      messageGeneric:
        '¿Seguro que quieres eliminar este interés? Esta acción no se puede deshacer.',
      success: '"{name}" ha sido eliminado.',
      deletedSuccess: 'Interés eliminado correctamente.',
      error: 'No se pudo eliminar el interés. Inténtalo de nuevo.',
    },

    messages: {
      updated: 'Interés actualizado correctamente',
      created: 'Interés creado correctamente',
      updateError: 'Fallo al actualizar el interés',
      createError: 'Fallo al crear el interés',
      loadError: 'No se pudo cargar el interés',
      loadListError: 'No se pudieron cargar los intereses. Inténtalo de nuevo.',
    },
  },

  email: {
    titles: {
      memberEmails: 'Correos',
      todayEmails: 'Correos de Hoy',
      sendToAll: 'Enviar Correo a Todos los Miembros',
      sendByInterest: 'Enviar Correo por Interés',
      quota: 'Cuota de Correo',
      previewFallback: 'Correo',
    },

    templates: {
      none: 'Sin Plantilla',
      basic: 'Básica',
    },

    descriptions: {
      sendToAll: 'Envía un correo a todos los miembros de la base de datos.',
      sendByInterest: 'Envía un correo a todos los miembros que compartan uno o más intereses.',
      preview: 'Así es como verá el correo el destinatario.',
    },

    memberEmailsCard: {
      refresh: 'Actualizar Correos',
      sendEmail: 'Enviar Correo',
      viewEmail: 'Ver Correo',
      empty: '@:common.states.noEmailsSentYet',
      errors: {
        load: 'No se pudieron cargar los correos.',
        loadContent: 'No se pudo cargar el contenido del correo.',
      },
    },

    todayEmailsCard: {
      refresh: '@:common.actions.refresh',
      viewEmail: 'Ver Correo',
      empty: '@:common.states.noEmailsSentYet',
      errors: {
        load: 'No se pudieron cargar los correos de hoy.',
        loadContent: 'No se pudo cargar el contenido del correo.',
      },
    },

    sendEmailDialog: {
      titles: {
        preview: 'Vista Previa — {subject}',
        byInterest: 'Enviar Correo por Interés',
        toAll: 'Enviar Correo a todos los Miembros',
        default: 'Enviar Correo',
      },
      help: {
        interests:
          'El correo se enviará a todos los miembros que tengan al menos uno de los intereses seleccionados.',
      },
      attachments: {
        empty: 'No hay archivos seleccionados — máximo 5 MB en total',
        summary: '{count} archivo(s) — {size} / 5 MB',
      },
      actions: {
        backToEdit: 'Volver a Editar',
      },
      preview: {
        iframeTitle: 'Vista Previa del Correo',
      },
      validation: {
        subjectRequired: 'El asunto es obligatorio',
        bodyRequired: 'El contenido es obligatorio',
        interestsRequired: '@:common.validation.interestsRequired',
        attachmentLimitExceeded: 'El tamaño total supera el límite de 5 MB ({size} seleccionados)',
      },
      messages: {
        sentDetail: 'Correo enviado correctamente.',
        sendError: 'No se pudo enviar el correo.',
      },
    },

    quotaCard: {
      refresh: 'Actualizar Estado',
      sentToday: '{sent} / {limit} enviados hoy',
      noRemaining: 'No quedan correos para hoy',
      remaining: 'Correos Restantes',
      resetsAt: '@:common.states.resetsAtMidnight',
      errors: {
        load: 'No se pudo cargar el estado del proveedor de correo.',
      },
    },

    previewDialog: {
      iframeTitle: 'Vista Previa del Cuerpo del Correo',
    },
  },

  genders: {
    selectorPlaceholder: '@:common.placeholders.searchGender',
  },

  countries: {
    selectorPlaceholder: '@:common.placeholders.searchCountry',
  },

  tours: {
    common: {
      next: 'Siguiente',
      previous: 'Anterior',
      done: 'Finalizar',
      close: 'Cerrar',
      progress: '{current} de {total}',
    },

    tools: {
      quota: {
        title: 'Cuota de Correos',
        description:
          'Muestra cuántos correos has enviado y cuántos te quedan en el periodo actual.',
      },
      sendByInterest: {
        title: 'Enviar por Interés',
        description: 'Envía un correo solo a los socios suscritos a un grupo de interés concreto.',
      },
      sendToAll: {
        title: 'Enviar a Todos',
        description:
          'Envía un correo a todos los socios a la vez. Úsalo con cuidado: llega a toda la lista.',
      },
      versionChip: {
        title: 'Versión de la Aplicación',
        description:
          'Muestra la versión de la aplicación que estás usando, así como la versión del servidor.',
      },
    },

    home: {
      birthdays: {
        title: 'Cumpleaños de hoy',
        description: 'Muestra los socios que cumplen años hoy.',
      },
      emailsToday: {
        title: 'Correos enviados hoy',
        description: 'Muestra los correos que se han enviado hoy.',
      },
    },

    members: {
      search: {
        title: 'Buscar socios',
        description: 'Filtra la lista por nombre, correo o teléfono mientras escribes.',
      },
      interestFilter: {
        title: 'Filtrar por interés',
        description: 'Muestra solo los socios suscritos a uno o varios grupos de interés.',
      },
      export: {
        title: 'Exportar',
        description: 'Descarga la lista filtrada actual como archivo.',
      },
      add: {
        title: 'Añadir socio',
        description: 'Crea un nuevo registro de socio.',
      },
      table: {
        title: 'Lista de socios',
        description:
          'Haz clic en una fila para editar ese socio, o usa el icono de papelera para eliminarlo.',
      },

      upsert: {
        personalDetails: {
          title: 'Datos personales',
          description: 'Introduce el nombre, apellidos, fecha de nacimiento y género del socio.',
        },
        contact: {
          title: 'Información de contacto',
          description: 'Añade el correo, el teléfono y el país para contactar con este socio.',
        },
        interests: {
          title: 'Intereses',
          description:
            'Asigna uno o varios grupos de interés para que este socio reciba correos relevantes.',
        },
        emailsHistory: {
          title: 'Historial de correos',
          description: 'Muestra todos los correos que ha recibido este socio hasta ahora.',
        },
        save: {
          title: 'Guardar',
          description: 'Guarda el socio. Los campos obligatorios deben estar rellenados primero.',
        },
        delete: {
          title: 'Eliminar socio',
          description: 'Elimina este socio de forma permanente. Esta acción no se puede deshacer.',
        },
      },
    },

    users: {
      list: {
        search: {
          title: 'Buscar usuarios',
          description: 'Filtra la lista por nombre o correo mientras escribes.',
        },
        add: {
          title: 'Añadir usuario',
          description: 'Crea una nueva cuenta de usuario.',
        },
        table: {
          title: 'Lista de usuarios',
          description:
            'Haz clic en una fila para editar ese usuario. La etiqueta indica su rol: administrador o usuario normal.',
        },
      },

      upsert: {
        form: {
          title: 'Datos del usuario',
          description: 'Introduce el nombre y el correo electrónico del usuario.',
        },
        passwordField: {
          title: 'Establecer contraseña',
          description: 'Elige una contraseña inicial para la cuenta de este usuario.',
        },
        save: {
          title: 'Guardar',
          description:
            'Guarda este usuario. Los campos obligatorios deben estar rellenados primero.',
        },
        passwordCard: {
          title: 'Cambiar contraseña',
          description: 'Establece una nueva contraseña para esta cuenta.',
        },
        roleCard: {
          title: 'Rol',
          description: 'Asciende a este usuario a administrador, o vuélvelo a usuario normal.',
        },
        delete: {
          title: 'Eliminar usuario',
          description:
            'Elimina de forma permanente la cuenta de este usuario. Esta acción no se puede deshacer.',
        },
      },
    },

    interests: {
      list: {
        search: {
          title: 'Buscar intereses',
          description: 'Filtra la lista por nombre o descripción mientras escribes.',
        },
        add: {
          title: 'Añadir interés',
          description: 'Crea un nuevo grupo de interés al que los socios puedan suscribirse.',
        },
        table: {
          title: 'Lista de intereses',
          description:
            'Haz clic en una fila para editar ese interés, o usa el icono de papelera para eliminarlo.',
        },
      },
    },
  },
}
