//en.ts
export default {
  common: {
    cancel: 'Cancel',
    saveChanges: 'Save Changes',
    apply: 'Apply',
    error: 'Error',
    saved: 'Saved',
    created: 'Created',
    deleted: 'Deleted',
  },
  users: {
    roles: {
      USER: 'User',
      ADMIN: 'Admin',
    },
    actions: {
      goBack: 'Go back',
      deleteUser: 'Delete user',
    },
    page: {
      editTitle: 'Edit User',
      newTitle: 'New User',
      editDescription: "Update the user's details below",
      newDescription: 'Fill in the details to create a new user',
    },
    form: {
      name: {
        label: 'Name',
        placeholder: 'Enter name',
        required: 'Name is required',
        max: 'Name must not exceed 100 characters',
      },
      email: {
        label: 'Email',
        placeholder: 'Enter email',
        required: 'Email is required',
        invalid: 'Email must be valid',
      },
      password: {
        label: 'Password',
        placeholder: 'Enter password',
        required: 'Password is required',
        min: 'Password must be at least 8 characters',
      },
      submitCreate: 'Create User',
    },
    passwordCard: {
      title: 'Change Password',
      currentPassword: {
        label: 'Current Password',
        placeholder: 'Enter current password',
        required: 'Current password is required',
      },
      newPassword: {
        label: 'New Password',
        placeholder: 'Enter new password',
        required: 'New password is required',
        min: 'Password must be at least 8 characters',
      },
      confirmPassword: {
        label: 'Confirm New Password',
        placeholder: 'Repeat new password',
        required: 'Please confirm the new password',
        mismatch: 'Passwords do not match',
      },
      submit: 'Update Password',
      updatedTitle: 'Password updated',
      updatedDetail: 'Password changed successfully.',
      updateError: 'Failed to update password. Check your current password and try again.',
    },
    roleCard: {
      title: 'Role',
      description: 'Changing this will immediately affect what the user can access.',
      updatedTitle: 'Role updated',
      updatedDetail: 'Role changed to {role}',
      updateError: 'Failed to update role',
    },
    deleteDialog: {
      header: 'Delete User',
      message: 'Are you sure you want to delete this user? This action cannot be undone.',
      accept: 'Delete',
      reject: 'Cancel',
      success: 'User deleted successfully.',
      error: 'Failed to delete user. Please try again.',
    },
    toasts: {
      updated: 'User updated successfully',
      created: 'User created successfully',
      updateError: 'Failed to update user',
      createError: 'Failed to create user',
      loadError: 'Failed to load user',
    },
    list: {
      page: {
        title: 'Users',
        totalUsers: '{total} total users',
      },
      filters: {
        searchPlaceholder: 'Search users...',
      },
      actions: {
        newUser: 'New User',
      },
      table: {
        name: 'Name',
        email: 'Email',
        role: 'Role',
        createdAt: 'Created At',
      },
      empty: 'No users found',
      toasts: {
        loadError: 'Failed to load users. Please try again.',
      },
    },
  },
  tools: {
    page: {
      title: 'Tools',
      description: 'Various tools and utilities',
    },
  },
  home: {
    page: {
      title: 'Dashboard',
      description: 'Welcome back',
    },
  },
  members: {
    components: {
      todayBirthdayCard: {
        title: "Today's Birthdays",
        actions: {
          refresh: 'Refresh',
        },
        table: {
          member: 'Member',
          email: 'Email',
          phone: 'Phone',
          birthday: 'Birthday',
          emptyPhone: '—',
        },
        pagination: {
          report: '{first} to {last} of {totalRecords}',
        },
        empty: 'No birthdays today',
        toasts: {
          loadError: "Failed to load today's birthdays.",
        },
      },
    },
    actions: {
      deleteMember: 'Delete member',
      goBack: 'Go back',
    },
    page: {
      editTitle: 'Edit Member',
      newTitle: 'New Member',
      editDescription: "Update the member's details below",
      newDescription: 'Fill in the details to create a new member',
    },
    sections: {
      personalDetails: 'Personal Details',
      contact: 'Contact',
      notes: 'Notes',
      interests: 'Interests',
    },
    form: {
      name: {
        label: 'Name',
        placeholder: 'Enter name',
        required: 'Name is required',
        max: 'Name must not exceed 100 characters',
      },
      surname: {
        label: 'Surname',
        placeholder: 'Enter surname',
        required: 'Surname is required',
        max: 'Surname must not exceed 100 characters',
      },
      secondSurname: {
        label: 'Second Surname',
        placeholder: 'Enter second surname',
        max: 'Second surname must not exceed 100 characters',
      },
      email: {
        label: 'Email',
        placeholder: 'Enter email',
        required: 'Email is required',
        invalid: 'Email must be valid',
        editWarning:
          'Changing the email will hide previously sent emails, as they are matched by email address.',
      },
      birthdate: {
        label: 'Birthdate',
      },
      gender: {
        label: 'Gender',
      },
      phone: {
        label: 'Phone',
        placeholder: 'Enter phone',
        max: 'Phone must not exceed 30 characters',
      },
      country: {
        label: 'Country',
      },
      notes: {
        label: 'Notes',
        placeholder: 'Any additional notes',
      },
      interests: {
        label: 'Interests',
        help: 'Select one or more interests for this member',
      },
      submitCreate: 'Create Member',
    },
    list: {
      page: {
        title: 'Members',
        totalMembers: '{total} total members',
      },
      filters: {
        searchPlaceholder: 'Search members...',
        interestsPlaceholder: 'Filter by interest...',
        interestsSearchPlaceholder: 'Search...',
      },
      actions: {
        export: 'Export',
        newMember: 'New Member',
      },
      table: {
        name: 'Name',
        email: 'Email',
        phone: 'Phone',
        birthdate: 'Birthdate',
        createdAt: 'Created At',
      },
      empty: 'No members found',
      toasts: {
        loadError: 'Failed to load members. Please try again.',
        exportError: 'Failed to export members.',
      },
    },
    deleteDialog: {
      header: 'Delete Member',
      message: 'Are you sure you want to delete {name}? This action cannot be undone.',
      messageGeneric: 'Are you sure you want to delete this member? This action cannot be undone.',
      accept: 'Delete',
      reject: 'Cancel',
      success: '{name} has been deleted.',
      deletedSuccess: 'Member deleted successfully.',
      error: 'Failed to delete member. Please try again.',
    },
    toasts: {
      updated: 'Member updated successfully',
      created: 'Member created successfully',
      updateError: 'Failed to update member',
      createError: 'Failed to create member',
      loadError: 'Failed to load member',
    },
  },
  interests: {
    components: {
      multiSelect: {
        placeholder: 'Select interests...',
        filterPlaceholder: 'Search interests...',
      },
    },
    actions: {
      goBack: 'Go back',
      deleteInterest: 'Delete interest',
    },
    page: {
      editTitle: 'Edit Interest',
      newTitle: 'New Interest',
      editDescription: "Update the interest's details below",
      newDescription: 'Fill in the details to create a new interest',
    },
    form: {
      name: {
        label: 'Name',
        placeholder: 'Enter interest name',
        required: 'Name is required',
        max: 'Name must not exceed 100 characters',
      },
      description: {
        label: 'Description',
        placeholder: 'Enter a description',
      },
      submitCreate: 'Create Interest',
    },
    list: {
      page: {
        title: 'Interests',
        count: 'No interests | {count} interest | {count} interests',
      },
      filters: {
        searchPlaceholder: 'Search interests...',
      },
      actions: {
        newInterest: 'New Interest',
      },
      table: {
        name: 'Name',
        description: 'Description',
        emptyDescription: '—',
      },
      empty: 'No interests found',
      toasts: {
        loadError: 'Failed to load interests. Please try again.',
      },
    },
    deleteDialog: {
      header: 'Delete Interest',
      message: 'Are you sure you want to delete "{name}"? This action cannot be undone.',
      messageGeneric:
        'Are you sure you want to delete this interest? This action cannot be undone.',
      accept: 'Delete',
      reject: 'Cancel',
      success: '"{name}" has been deleted.',
      deletedSuccess: 'Interest deleted successfully.',
      error: 'Failed to delete interest. Please try again.',
    },
    toasts: {
      updated: 'Interest updated successfully',
      created: 'Interest created successfully',
      updateError: 'Failed to update interest',
      createError: 'Failed to create interest',
      loadError: 'Failed to load interest',
    },
  },
  auth: {
    login: {
      page: {
        title: 'Sign in',
        description: 'Welcome back — please enter your details',
      },
      form: {
        email: {
          label: 'Email',
          placeholder: 'Enter your email',
          required: 'Email is required',
          invalid: 'Must be a valid email',
        },
        password: {
          label: 'Password',
          placeholder: 'Enter your password',
          required: 'Password is required',
        },
        submit: 'Sign in',
      },
      toasts: {
        loginError: 'Login failed',
      },
    },
  },
  navigation: {
    nav: {
      users: 'Users',
      members: 'Members',
      interests: 'Interests',
      tools: 'Tools',
    },
    actions: {
      signOut: 'Sign out',
      switchToLightMode: 'Switch to light mode',
      switchToDarkMode: 'Switch to dark mode',
      toggleMenu: 'Toggle menu',
    },
  },
  email: {
    components: {
      memberEmailsCard: {
        title: 'Emails',
        actions: {
          refresh: 'Refresh emails',
          sendEmail: 'Send Email',
          viewEmail: 'View email',
        },
        table: {
          subject: 'Subject',
          provider: 'Provider',
          sentAt: 'Sent At',
        },
        empty: 'No emails sent yet',
        toasts: {
          loadError: 'Failed to load emails.',
          loadContentError: 'Failed to load email content.',
        },
      },
      todayEmailsCard: {
        title: "Today's Emails",
        actions: {
          refresh: 'Refresh',
          viewEmail: 'View email',
        },
        table: {
          subject: 'Subject',
          provider: 'Provider',
          sentTo: 'Sent To',
          sentAt: 'Sent At',
        },
        empty: 'No emails sent yet',
        toasts: {
          loadError: "Failed to load today's emails.",
          loadContentError: 'Failed to load email content.',
        },
      },
      sendEmailsToAllCard: {
        title: 'Email All Members',
        description: 'Send an email to every member in the database.',
        actions: {
          compose: 'Compose Email',
        },
      },
      sendEmailDialog: {
        titles: {
          preview: 'Preview — {subject}',
          byInterest: 'Send Email by Interest',
          toAll: 'Send Email to All Members',
          default: 'Send Email',
        },
        fields: {
          template: 'Template',
          interests: 'Interests',
          subject: 'Subject',
          body: 'Body',
          attachments: 'Attachments',
        },
        placeholders: {
          subject: 'Enter email subject',
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
          addFiles: 'Add files',
          removeAttachment: 'Remove attachment',
          preview: 'Preview',
          backToEdit: 'Back to edit',
          send: 'Send',
        },
        preview: {
          description: 'This is how the email will look to the recipient.',
          iframeTitle: 'Email preview',
        },
        validation: {
          subjectRequired: 'Subject is required',
          bodyRequired: 'Body is required',
          interestsRequired: 'Select at least one interest',
          attachmentLimitExceeded: 'Total size exceeds the 5 MB limit ({size} selected)',
        },
        toasts: {
          sentTitle: 'Sent',
          sentDetail: 'Email sent successfully.',
          sendError: 'Failed to send email.',
        },
      },
      sendEmailByInterestCard: {
        title: 'Email by Interest',
        description: 'Send an email to all members that share one or more interests.',
        actions: {
          compose: 'Compose Email',
        },
      },
      quotaCard: {
        title: 'Email Quota',
        actions: {
          refresh: 'Refresh status',
        },
        labels: {
          provider: 'Provider',
          sentToday: '{sent} / {limit} sent today',
        },
        status: {
          noneRemaining: 'No emails left today',
          remaining: 'emails remaining today',
          resetsAt: 'Resets at midnight',
        },
        toasts: {
          loadError: 'Failed to load email provider status.',
        },
      },
      previewDialog: {
        fallbackTitle: 'Email',
        fields: {
          sentAt: 'Sent At',
          provider: 'Provider',
          recipients: 'Recipients',
          body: 'Body',
        },
        iframeTitle: 'Email body preview',
        actions: {
          close: 'Close',
        },
      },
    },
  },
  genders: {
    components: {
      selector: {
        placeholder: 'Search gender...',
      },
    },
  },
  countries: {
    components: {
      selector: {
        placeholder: 'Search country...',
      },
    },
  },
}
