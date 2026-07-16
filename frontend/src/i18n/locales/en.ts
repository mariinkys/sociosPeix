export default {
  common: {
    actions: {
      cancel: 'Cancel',
      close: 'Close',
      apply: 'Apply',
      saveChanges: 'Save Changes',
      refresh: 'Refresh',
      back: 'Go back',
      create: 'Create',
      delete: 'Delete',
      export: 'Export',
      send: 'Send',
      preview: 'Preview',
      addFiles: 'Add files',
      removeAttachment: 'Remove attachment',
      composeEmail: 'Compose Email',
      signOut: 'Sign out',
      help: 'Help',
    },

    feedback: {
      error: 'Error',
      saved: 'Saved',
      created: 'Created',
      deleted: 'Deleted',
      sent: 'Sent',
    },

    fields: {
      name: 'Name',
      surname: 'Surname',
      secondSurname: 'Second Surname',
      email: 'Email',
      password: 'Password',
      currentPassword: 'Current Password',
      newPassword: 'New Password',
      confirmPassword: 'Confirm New Password',
      description: 'Description',
      phone: 'Phone',
      birthdate: 'Birthdate',
      gender: 'Gender',
      country: 'Country',
      notes: 'Notes',
      interests: 'Interests',
      role: 'Role',
      subject: 'Subject',
      body: 'Body',
      provider: 'Provider',
      recipients: 'Recipients',
      template: 'Template',
      attachments: 'Attachments',
      member: 'Member',
      birthday: 'Birthday',
      sentAt: 'Sent At',
      sentTo: 'Sent To',
      createdAt: 'Created At',
    },

    placeholders: {
      enterName: 'Enter name',
      enterSurname: 'Enter surname',
      enterSecondSurname: 'Enter second surname',
      enterEmail: 'Enter email',
      enterYourEmail: 'Enter your email',
      enterPassword: 'Enter password',
      enterCurrentPassword: 'Enter current password',
      enterNewPassword: 'Enter new password',
      repeatNewPassword: 'Repeat new password',
      enterPhone: 'Enter phone',
      enterDescription: 'Enter a description',
      enterInterestName: 'Enter interest name',
      enterEmailSubject: 'Enter email subject',
      additionalNotes: 'Any additional notes',
      search: 'Search...',
      searchUsers: 'Search users...',
      searchMembers: 'Search members...',
      searchInterests: 'Search interests...',
      searchGender: 'Search gender...',
      searchCountry: 'Search country...',
      filterByInterest: 'Filter by interest...',
      selectInterests: 'Select interests...',
    },

    validation: {
      required: 'This field is required',
      emailRequired: 'Email is required',
      passwordRequired: 'Password is required',
      invalidEmail: 'Must be a valid email',
      emailInvalid: 'Email must be valid',
      nameRequired: 'Name is required',
      surnameRequired: 'Surname is required',
      currentPasswordRequired: 'Current password is required',
      newPasswordRequired: 'New password is required',
      confirmNewPasswordRequired: 'Please confirm the new password',
      interestsRequired: 'Select at least one interest',
      passwordsMismatch: 'Passwords do not match',
      max100: 'Must not exceed 100 characters',
      min8: 'Must be at least 8 characters',
    },

    states: {
      emptyValue: '—',
      noResults: 'No results found',
      noEmailsSentYet: 'No emails sent yet',
      resetsAtMidnight: 'Resets at midnight',
    },

    theme: {
      switchToLightMode: 'Switch to light mode',
      switchToDarkMode: 'Switch to dark mode',
      toggleMenu: 'Toggle menu',
    },
  },

  home: {
    title: 'Dashboard',
    description: 'Welcome back',
  },

  tools: {
    title: 'Tools',
    description: 'Various tools and utilities',
  },

  navigation: {
    items: {
      users: 'Users',
      members: 'Members',
      interests: 'Interests',
      tools: 'Tools',
    },
  },

  auth: {
    login: {
      title: 'Sign in',
      description: 'Welcome back — please enter your details',
      submit: 'Sign in',
      errors: {
        loginFailed: 'Login failed',
      },
    },
  },

  users: {
    roles: {
      USER: 'User',
      ADMIN: 'Admin',
    },

    titles: {
      list: 'Users',
      create: 'New User',
      edit: 'Edit User',
    },

    descriptions: {
      create: 'Fill in the details to create a new user',
      edit: "Update the user's details below",
    },

    actions: {
      create: 'Create User',
      createNew: 'New User',
      delete: 'Delete user',
    },

    list: {
      total: '{total} total users',
      empty: 'No users found',
    },

    fields: {
      name: {
        label: '@:common.fields.name',
        placeholder: '@:common.placeholders.enterName',
        required: '@:common.validation.nameRequired',
        max: 'Name must not exceed 100 characters',
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
        min: 'Password must be at least 8 characters',
      },
    },

    passwordCard: {
      title: 'Change Password',
      submit: 'Update Password',
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
          min: 'Password must be at least 8 characters',
        },
        confirmPassword: {
          label: '@:common.fields.confirmPassword',
          placeholder: '@:common.placeholders.repeatNewPassword',
          required: '@:common.validation.confirmNewPasswordRequired',
          mismatch: '@:common.validation.passwordsMismatch',
        },
      },
      successTitle: 'Password updated',
      successDetail: 'Password changed successfully.',
      error: 'Failed to update password. Check your current password and try again.',
    },

    roleCard: {
      title: '@:common.fields.role',
      description: 'Changing this will immediately affect what the user can access.',
      updatedTitle: 'Role updated',
      updatedDetail: 'Role changed to {role}',
      updateError: 'Failed to update role',
    },

    deleteDialog: {
      title: 'Delete User',
      message: 'Are you sure you want to delete this user? This action cannot be undone.',
      success: 'User deleted successfully.',
      error: 'Failed to delete user. Please try again.',
    },

    messages: {
      updated: 'User updated successfully',
      created: 'User created successfully',
      updateError: 'Failed to update user',
      createError: 'Failed to create user',
      loadError: 'Failed to load user',
      loadListError: 'Failed to load users. Please try again.',
    },
  },

  members: {
    titles: {
      list: 'Members',
      create: 'New Member',
      edit: 'Edit Member',
    },

    descriptions: {
      create: 'Fill in the details to create a new member',
      edit: "Update the member's details below",
    },

    actions: {
      create: 'Create Member',
      createNew: 'New Member',
      delete: 'Delete member',
    },

    sections: {
      personalDetails: 'Personal Details',
      contact: 'Contact',
      notes: 'Notes',
      interests: 'Interests',
    },

    list: {
      total: '{total} total members',
      empty: 'No members found',
      exportError: 'Failed to export members.',
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
        max: 'Name must not exceed 100 characters',
      },
      surname: {
        label: '@:common.fields.surname',
        placeholder: '@:common.placeholders.enterSurname',
        required: '@:common.validation.surnameRequired',
        max: 'Surname must not exceed 100 characters',
      },
      secondSurname: {
        label: '@:common.fields.secondSurname',
        placeholder: '@:common.placeholders.enterSecondSurname',
        max: 'Second surname must not exceed 100 characters',
      },
      email: {
        label: '@:common.fields.email',
        placeholder: '@:common.placeholders.enterEmail',
        required: '@:common.validation.emailRequired',
        invalid: '@:common.validation.emailInvalid',
        editWarning:
          'Changing the email will hide previously sent emails, as they are matched by email address.',
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
        max: 'Phone must not exceed 30 characters',
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
        help: 'Select one or more interests for this member',
      },
    },

    todayBirthdayCard: {
      title: "Today's Birthdays",
      empty: 'No birthdays today',
      errors: {
        load: "Failed to load today's birthdays.",
      },
    },

    deleteDialog: {
      title: 'Delete Member',
      message: 'Are you sure you want to delete {name}? This action cannot be undone.',
      messageGeneric: 'Are you sure you want to delete this member? This action cannot be undone.',
      success: '{name} has been deleted.',
      deletedSuccess: 'Member deleted successfully.',
      error: 'Failed to delete member. Please try again.',
    },

    messages: {
      updated: 'Member updated successfully',
      created: 'Member created successfully',
      updateError: 'Failed to update member',
      createError: 'Failed to create member',
      loadError: 'Failed to load member',
      loadListError: 'Failed to load members. Please try again.',
    },
  },

  interests: {
    titles: {
      list: 'Interests',
      create: 'New Interest',
      edit: 'Edit Interest',
    },

    descriptions: {
      create: 'Fill in the details to create a new interest',
      edit: "Update the interest's details below",
    },

    actions: {
      create: 'Create Interest',
      createNew: 'New Interest',
      delete: 'Delete interest',
    },

    list: {
      count: 'No interests | {count} interest | {count} interests',
      empty: 'No interests found',
    },

    fields: {
      name: {
        label: '@:common.fields.name',
        placeholder: '@:common.placeholders.enterInterestName',
        required: '@:common.validation.nameRequired',
        max: 'Name must not exceed 100 characters',
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
      title: 'Delete Interest',
      message: 'Are you sure you want to delete "{name}"? This action cannot be undone.',
      messageGeneric:
        'Are you sure you want to delete this interest? This action cannot be undone.',
      success: '"{name}" has been deleted.',
      deletedSuccess: 'Interest deleted successfully.',
      error: 'Failed to delete interest. Please try again.',
    },

    messages: {
      updated: 'Interest updated successfully',
      created: 'Interest created successfully',
      updateError: 'Failed to update interest',
      createError: 'Failed to create interest',
      loadError: 'Failed to load interest',
      loadListError: 'Failed to load interests. Please try again.',
    },
  },

  email: {
    titles: {
      memberEmails: 'Emails',
      todayEmails: "Today's Emails",
      sendToAll: 'Email All Members',
      sendByInterest: 'Email by Interest',
      quota: 'Email Quota',
      previewFallback: 'Email',
    },

    templates: {
      none: 'No Template',
      basic: 'Basic',
    },

    descriptions: {
      sendToAll: 'Send an email to every member in the database.',
      sendByInterest: 'Send an email to all members that share one or more interests.',
      preview: 'This is how the email will look to the recipient.',
    },

    memberEmailsCard: {
      refresh: 'Refresh emails',
      sendEmail: 'Send Email',
      viewEmail: 'View email',
      empty: '@:common.states.noEmailsSentYet',
      errors: {
        load: 'Failed to load emails.',
        loadContent: 'Failed to load email content.',
      },
    },

    todayEmailsCard: {
      refresh: '@:common.actions.refresh',
      viewEmail: 'View email',
      empty: '@:common.states.noEmailsSentYet',
      errors: {
        load: "Failed to load today's emails.",
        loadContent: 'Failed to load email content.',
      },
    },

    sendEmailDialog: {
      titles: {
        preview: 'Preview — {subject}',
        byInterest: 'Send Email by Interest',
        toAll: 'Send Email to All Members',
        default: 'Send Email',
      },
      help: {
        interests:
          'The email will be sent to all members that have at least one of the selected interests.',
      },
      attachments: {
        empty: 'No files selected — max 5 MB total',
        summary: '{count} file(s) — {size} / 5 MB',
      },
      actions: {
        backToEdit: 'Back to edit',
      },
      preview: {
        iframeTitle: 'Email preview',
      },
      validation: {
        subjectRequired: 'Subject is required',
        bodyRequired: 'Body is required',
        interestsRequired: '@:common.validation.interestsRequired',
        attachmentLimitExceeded: 'Total size exceeds the 5 MB limit ({size} selected)',
      },
      messages: {
        sentDetail: 'Email sent successfully.',
        sendError: 'Failed to send email.',
      },
    },

    quotaCard: {
      refresh: 'Refresh Status',
      sentToday: '{sent} / {limit} sent today',
      noRemaining: 'No Emails Left Today',
      remaining: 'Emails Remaining Today',
      resetsAt: '@:common.states.resetsAtMidnight',
      errors: {
        load: 'Failed to load email provider status.',
      },
    },

    previewDialog: {
      iframeTitle: 'Email Body Preview',
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
      next: 'Next',
      previous: 'Previous',
      done: 'Done',
      close: 'Close',
      progress: '{current} of {total}',
    },

    tools: {
      quota: {
        title: 'Email Quota',
        description:
          "Shows how many emails you've sent and how many you have left in the current period.",
      },
      sendByInterest: {
        title: 'Send by Interest',
        description: 'Email only the members subscribed to a specific interest group.',
      },
      sendToAll: {
        title: 'Send to Everyone',
        description:
          'Broadcast an email to every member at once. Use with care, this reaches your whole list.',
      },
      versionChip: {
        title: 'App Version',
        description:
          "Shows the version of the app you're currently using, as well as the server version.",
      },
    },

    home: {
      birthdays: {
        title: "Today's birthdays",
        description: 'Shows the members who have their birthday today.',
      },
      emailsToday: {
        title: 'Emails sent today',
        description: 'Shows the emails that have been sent out today.',
      },
    },
  },
}
