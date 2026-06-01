//es.ts
export default {
  common: {
    cancel: 'Cancelar',
    saveChanges: 'Guardar cambios',
    apply: 'Aplicar',
    error: 'Error',
    saved: 'Guardado',
    created: 'Creado',
    deleted: 'Eliminado',
  },
  users: {
    roles: {
      USER: 'Usuario',
      ADMIN: 'Administrador',
    },
    actions: {
      goBack: 'Volver',
      deleteUser: 'Eliminar usuario',
    },
    page: {
      editTitle: 'Editar usuario',
      newTitle: 'Nuevo usuario',
      editDescription: 'Actualiza los datos del usuario a continuación',
      newDescription: 'Completa los datos para crear un nuevo usuario',
    },
    form: {
      name: {
        label: 'Nombre',
        placeholder: 'Introduce el nombre',
        required: 'El nombre es obligatorio',
        max: 'El nombre no debe pasar de 100 caracteres',
      },
      email: {
        label: 'Correo electrónico',
        placeholder: 'Introduce el correo electrónico',
        required: 'El correo electrónico es obligatorio',
        invalid: 'El correo electrónico debe ser válido',
      },
      password: {
        label: 'Contraseña',
        placeholder: 'Introduce la contraseña',
        required: 'La contraseña es obligatoria',
        min: 'La contraseña debe tener al menos 8 caracteres',
      },
      submitCreate: 'Crear usuario',
    },
    passwordCard: {
      title: 'Cambiar contraseña',
      currentPassword: {
        label: 'Contraseña actual',
        placeholder: 'Introduce la contraseña actual',
        required: 'La contraseña actual es obligatoria',
      },
      newPassword: {
        label: 'Nueva contraseña',
        placeholder: 'Introduce la nueva contraseña',
        required: 'La nueva contraseña es obligatoria',
        min: 'La contraseña debe tener al menos 8 caracteres',
      },
      confirmPassword: {
        label: 'Confirmar nueva contraseña',
        placeholder: 'Repite la nueva contraseña',
        required: 'Por favor, confirma la nueva contraseña',
        mismatch: 'Las contraseñas no coinciden',
      },
      submit: 'Actualizar contraseña',
      updatedTitle: 'Contraseña actualizada',
      updatedDetail: 'La contraseña se cambió correctamente.',
      updateError:
        'No se pudo actualizar la contraseña. Comprueba tu contraseña actual e inténtalo de nuevo.',
    },
    roleCard: {
      title: 'Rol',
      description: 'Cambiar esto afectará inmediatamente a lo que el usuario puede acceder.',
      updatedTitle: 'Rol actualizado',
      updatedDetail: 'Rol cambiado a {role}',
      updateError: 'No se pudo actualizar el rol',
    },
    deleteDialog: {
      header: 'Eliminar usuario',
      message: '¿Seguro que quieres eliminar este usuario? Esta acción no se puede deshacer.',
      accept: 'Eliminar',
      reject: 'Cancelar',
      success: 'Usuario eliminado correctamente.',
      error: 'No se pudo eliminar el usuario. Inténtalo de nuevo.',
    },
    toasts: {
      updated: 'Usuario actualizado correctamente',
      created: 'Usuario creado correctamente',
      updateError: 'Fallo al actualizar el usuario',
      createError: 'Fallo al crear el usuario',
      loadError: 'No se pudo cargar el usuario',
    },
    list: {
      page: {
        title: 'Usuarios',
        totalUsers: '{total} usuarios en total',
      },
      filters: {
        searchPlaceholder: 'Buscar usuarios...',
      },
      actions: {
        newUser: 'Nuevo usuario',
      },
      table: {
        name: 'Nombre',
        email: 'Correo electrónico',
        role: 'Rol',
        createdAt: 'Creado el',
      },
      empty: 'No se encontraron usuarios',
      toasts: {
        loadError: 'No se pudieron cargar los usuarios. Inténtalo de nuevo.',
      },
    },
  },
  tools: {
    page: {
      title: 'Herramientas',
      description: 'Varias herramientas y utilidades',
    },
  },
  home: {
    page: {
      title: 'Dashboard',
      description: 'Bienvenido/a de vuelta',
    },
  },
  members: {
    components: {
      todayBirthdayCard: {
        title: 'Cumpleaños de hoy',
        actions: {
          refresh: 'Actualizar',
        },
        table: {
          member: 'Miembro',
          email: 'Correo electrónico',
          phone: 'Teléfono',
          birthday: 'Cumpleaños',
          emptyPhone: '—',
        },
        empty: 'No hay cumpleaños hoy',
        toasts: {
          loadError: 'No se pudieron cargar los cumpleaños de hoy.',
        },
      },
    },
    actions: {
      goBack: 'Volver',
      deleteMember: 'Eliminar miembro',
    },
    page: {
      editTitle: 'Editar miembro',
      newTitle: 'Nuevo miembro',
      editDescription: 'Actualiza los datos del miembro a continuación',
      newDescription: 'Completa los datos para crear un nuevo miembro',
    },
    sections: {
      personalDetails: 'Datos personales',
      contact: 'Contacto',
      notes: 'Notas',
      interests: 'Intereses',
    },
    form: {
      name: {
        label: 'Nombre',
        placeholder: 'Introduce el nombre',
        required: 'El nombre es obligatorio',
        max: 'El nombre no debe pasar de 100 caracteres',
      },
      surname: {
        label: 'Apellido',
        placeholder: 'Introduce el apellido',
        required: 'El apellido es obligatorio',
        max: 'El apellido no debe pasar de 100 caracteres',
      },
      secondSurname: {
        label: 'Segundo apellido',
        placeholder: 'Introduce el segundo apellido',
        max: 'El segundo apellido no debe pasar de 100 caracteres',
      },
      email: {
        label: 'Correo electrónico',
        placeholder: 'Introduce el correo electrónico',
        required: 'El correo electrónico es obligatorio',
        invalid: 'El correo electrónico debe ser válido',
        editWarning:
          'Cambiar el correo ocultará los correos enviados anteriormente, ya que se relacionan por dirección de correo electrónico.',
      },
      birthdate: {
        label: 'Fecha de nacimiento',
      },
      gender: {
        label: 'Género',
      },
      phone: {
        label: 'Teléfono',
        placeholder: 'Introduce el teléfono',
        max: 'El teléfono no debe pasar de 30 caracteres',
      },
      country: {
        label: 'País',
      },
      notes: {
        label: 'Notas',
        placeholder: 'Cualquier nota adicional',
      },
      interests: {
        label: 'Intereses',
        help: 'Selecciona uno o más intereses para este miembro',
      },
      submitCreate: 'Crear miembro',
    },
    list: {
      page: {
        title: 'Miembros',
        totalMembers: '{total} miembros en total',
      },
      filters: {
        searchPlaceholder: 'Buscar miembros...',
        interestsPlaceholder: 'Filtrar por interés...',
        interestsSearchPlaceholder: 'Buscar...',
      },
      actions: {
        export: 'Exportar',
        newMember: 'Nuevo miembro',
      },
      table: {
        name: 'Nombre',
        email: 'Correo electrónico',
        phone: 'Teléfono',
        birthdate: 'Fecha de nacimiento',
        createdAt: 'Creado el',
      },
      empty: 'No se encontraron miembros',
      toasts: {
        loadError: 'No se pudieron cargar los miembros. Inténtalo de nuevo.',
        exportError: 'No se pudieron exportar los miembros.',
      },
    },
    deleteDialog: {
      header: 'Eliminar miembro',
      message: '¿Seguro que quieres eliminar a {name}? Esta acción no se puede deshacer.',
      messageGeneric:
        '¿Seguro que quieres eliminar este miembro? Esta acción no se puede deshacer.',
      accept: 'Eliminar',
      reject: 'Cancelar',
      success: 'Se ha eliminado a {name}.',
      deletedSuccess: 'Miembro eliminado correctamente.',
      error: 'No se pudo eliminar el miembro. Inténtalo de nuevo.',
    },
    toasts: {
      updated: 'Miembro actualizado correctamente',
      created: 'Miembro creado correctamente',
      updateError: 'No se pudo actualizar el miembro',
      createError: 'No se pudo crear el miembro',
      loadError: 'No se pudo cargar el miembro',
    },
  },
  interests: {
    components: {
      multiSelect: {
        placeholder: 'Selecciona intereses...',
        filterPlaceholder: 'Buscar intereses...',
      },
    },
    actions: {
      goBack: 'Volver',
      deleteInterest: 'Eliminar interés',
    },
    page: {
      editTitle: 'Editar interés',
      newTitle: 'Nuevo interés',
      editDescription: 'Actualiza los datos del interés a continuación',
      newDescription: 'Completa los datos para crear un nuevo interés',
    },
    form: {
      name: {
        label: 'Nombre',
        placeholder: 'Introduce el nombre del interés',
        required: 'El nombre es obligatorio',
        max: 'El nombre no debe pasar de 100 caracteres',
      },
      description: {
        label: 'Descripción',
        placeholder: 'Introduce una descripción',
      },
      submitCreate: 'Crear interés',
    },
    list: {
      page: {
        title: 'Intereses',
        count: 'No hay intereses | {count} interés | {count} intereses',
      },
      filters: {
        searchPlaceholder: 'Buscar intereses...',
      },
      actions: {
        newInterest: 'Nuevo interés',
      },
      table: {
        name: 'Nombre',
        description: 'Descripción',
        emptyDescription: '—',
      },
      empty: 'No se encontraron intereses',
      toasts: {
        loadError: 'No se pudieron cargar los intereses. Inténtalo de nuevo.',
      },
    },
    deleteDialog: {
      header: 'Eliminar interés',
      message: '¿Seguro que quieres eliminar "{name}"? Esta acción no se puede deshacer.',
      messageGeneric:
        '¿Seguro que quieres eliminar este interés? Esta acción no se puede deshacer.',
      accept: 'Eliminar',
      reject: 'Cancelar',
      success: 'Se ha eliminado "{name}".',
      deletedSuccess: 'Interés eliminado correctamente.',
      error: 'No se pudo eliminar el interés. Inténtalo de nuevo.',
    },
    toasts: {
      updated: 'Interés actualizado correctamente',
      created: 'Interés creado correctamente',
      updateError: 'No se pudo actualizar el interés',
      createError: 'No se pudo crear el interés',
      loadError: 'No se pudo cargar el interés',
    },
  },
  auth: {
    login: {
      page: {
        title: 'Iniciar sesión',
        description: 'Te damos la bienvenida de nuevo — introduce tus datos',
      },
      form: {
        email: {
          label: 'Correo electrónico',
          placeholder: 'Introduce tu correo electrónico',
          required: 'El correo electrónico es obligatorio',
          invalid: 'Debe ser un correo electrónico válido',
        },
        password: {
          label: 'Contraseña',
          placeholder: 'Introduce tu contraseña',
          required: 'La contraseña es obligatoria',
        },
        submit: 'Iniciar sesión',
      },
      toasts: {
        loginError: 'No se pudo iniciar sesión',
      },
    },
  },
  navigation: {
    nav: {
      users: 'Usuarios',
      members: 'Miembros',
      interests: 'Intereses',
      tools: 'Herramientas',
    },
    actions: {
      signOut: 'Cerrar sesión',
      switchToLightMode: 'Cambiar a modo claro',
      switchToDarkMode: 'Cambiar a modo oscuro',
      toggleMenu: 'Mostrar u ocultar menú',
    },
  },
  email: {
    components: {
      memberEmailsCard: {
        title: 'Correos',
        actions: {
          refresh: 'Actualizar correos',
          sendEmail: 'Enviar correo',
          viewEmail: 'Ver correo',
        },
        table: {
          subject: 'Asunto',
          provider: 'Proveedor',
          sentAt: 'Enviado el',
        },
        empty: 'Aún no se han enviado correos',
        toasts: {
          loadError: 'No se pudieron cargar los correos.',
          loadContentError: 'No se pudo cargar el contenido del correo.',
        },
      },
      todayEmailsCard: {
        title: 'Correos de hoy',
        actions: {
          refresh: 'Actualizar',
          viewEmail: 'Ver correo',
        },
        table: {
          subject: 'Asunto',
          provider: 'Proveedor',
          sentTo: 'Enviado a',
          sentAt: 'Enviado el',
        },
        empty: 'Aún no se han enviado correos',
        toasts: {
          loadError: 'No se pudieron cargar los correos de hoy.',
          loadContentError: 'No se pudo cargar el contenido del correo.',
        },
      },
      sendEmailsToAllCard: {
        title: 'Enviar correo a todos los miembros',
        description: 'Envía un correo electrónico a todos los miembros de la base de datos.',
        actions: {
          compose: 'Redactar correo',
        },
      },
      sendEmailDialog: {
        titles: {
          preview: 'Vista previa — {subject}',
          byInterest: 'Enviar correo por interés',
          toAll: 'Enviar correo a todos los miembros',
          default: 'Enviar correo',
        },
        fields: {
          template: 'Plantilla',
          interests: 'Intereses',
          subject: 'Asunto',
          body: 'Contenido',
          attachments: 'Archivos adjuntos',
        },
        placeholders: {
          subject: 'Introduce el asunto del correo',
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
          addFiles: 'Añadir archivos',
          removeAttachment: 'Eliminar archivo adjunto',
          preview: 'Vista previa',
          backToEdit: 'Volver a editar',
          send: 'Enviar',
        },
        preview: {
          description: 'Así es como verá el correo el destinatario.',
          iframeTitle: 'Vista previa del correo',
        },
        validation: {
          subjectRequired: 'El asunto es obligatorio',
          bodyRequired: 'El contenido es obligatorio',
          interestsRequired: 'Selecciona al menos un interés',
          attachmentLimitExceeded:
            'El tamaño total supera el límite de 5 MB ({size} seleccionados)',
        },
        toasts: {
          sentTitle: 'Enviado',
          sentDetail: 'Correo enviado correctamente.',
          sendError: 'No se pudo enviar el correo.',
        },
      },
      sendEmailByInterestCard: {
        title: 'Enviar correo por interés',
        description:
          'Envía un correo electrónico a todos los miembros que comparten uno o más intereses.',
        actions: {
          compose: 'Redactar correo',
        },
      },
      quotaCard: {
        title: 'Cuota de correos',
        actions: {
          refresh: 'Actualizar estado',
        },
        labels: {
          provider: 'Proveedor',
          sentToday: '{sent} / {limit} enviados hoy',
        },
        status: {
          noneRemaining: 'No quedan correos para hoy',
          remaining: 'correos restantes hoy',
          resetsAt: 'Se restablece a medianoche',
        },
        toasts: {
          loadError: 'No se pudo cargar el estado del proveedor de correo.',
        },
      },
      previewDialog: {
        fallbackTitle: 'Correo',
        fields: {
          sentAt: 'Enviado el',
          provider: 'Proveedor',
          recipients: 'Destinatarios',
          body: 'Contenido',
        },
        iframeTitle: 'Vista previa del contenido del correo',
        actions: {
          close: 'Cerrar',
        },
      },
    },
  },
  genders: {
    components: {
      selector: {
        placeholder: 'Buscar género...',
      },
    },
  },
  countries: {
    components: {
      selector: {
        placeholder: 'Buscar país...',
      },
    },
  },
}
