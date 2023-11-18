macro_rules! define_error_domain {
    ($domain:expr, { $($variant:ident($key:expr, $msg:expr)),* $(,)? }) => {
        #[derive(Debug)]
        #[allow(dead_code)]
        pub enum Error {
            Generic(Box<dyn std::error::Error>),
            $($variant),*
        }

        impl From<std::io::Error> for Error {
            fn from(err: std::io::Error) -> Self {
                Error::Generic(Box::new(err))
            }
        }

        impl From<serde_json::Error> for Error {
            fn from(err: serde_json::Error) -> Self {
                Error::Generic(Box::new(err))
            }
        }

        impl serde::Serialize for Error {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct("Error", 3)?;
                // Serialize the full error key with domain
                state.serialize_field("error", &self.error_key())?;
                // Serialize the message
                state.serialize_field("message", &self.default_message())?;
                // Placeholder for additional data
                state.serialize_field("data", &serde_json::Value::Null)?;
                state.end()
            }
        }

        impl Error {
            fn error_key(&self) -> String {
                match self {
                    Self::Generic(_) => format!("{}.generic_error", $domain),
                    $(Self::$variant => format!("{}.{}", $domain, $key)),*
                }
            }

            fn default_message(&self) -> String {
                match self {
                    Error::Generic(ref e) => e.to_string(),
                    $(Error::$variant => $msg.to_string()),*
                }
            }
            #[allow(dead_code)]
            pub fn generic<E>(error: E) -> Self
            where
                E: std::error::Error + 'static,
            {
                Self::Generic(Box::new(error))
            }
        }

        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", self.error_key(), self.default_message())
            }
        }

        impl std::error::Error for Error {}
    };
}

pub mod window_manager {
    define_error_domain!("window_manager", {
        ProcessNotFound("process_not_found", "Could not find a matching window for the profile."),
        ApplyFailed("apply_failed", "Failed to apply the window profile."),
        AccessDenied("access_denied", "Unable to apply the window profile due to access denied. Try running as administrator."),
        InvalidPID("invalid_pid", "The provided PID is invalid."),
    });
}

pub mod profile {
    define_error_domain!("profile", {
        NotFound("not_found", "Profile not found"),
        ProfilePathError("profile_path_error", "Could not find the profile path."),
    });
}
