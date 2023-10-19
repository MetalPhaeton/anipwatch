use log::error;

use std::{
    io::Error as IOError,
    cell::BorrowMutError
};

use image::ImageError;

use chobitlibs::{
    chobit_map::ChobitMapError,
    chobit_ani_value::ChobitAniValueError
};

use serde_yaml::Error as YamlError;

use crate::{
    settings_loader::LoadError,
    application::ApplicationError
};

#[derive(Debug)]
pub enum Error {
    IOError(IOError),
    ImageError(ImageError),
    ChobitMapError(ChobitMapError),
    ChobitAniValueError(ChobitAniValueError),
    YamlError(YamlError),
    NoSkin {id: u64},
    LoadError(LoadError),
    ApplicationError(ApplicationError),
    BorrowMutError(BorrowMutError)
}

impl From<IOError> for Error {
    #[inline]
    fn from(error: IOError) -> Self {
        Error::IOError(error)
    }
}

impl From<ImageError> for Error {
    #[inline]
    fn from(error: ImageError) -> Self {
        Error::ImageError(error)
    }
}

impl From<ChobitMapError> for Error {
    #[inline]
    fn from(error: ChobitMapError) -> Self {
        Error::ChobitMapError(error)
    }
}

impl From<ChobitAniValueError> for Error {
    #[inline]
    fn from(error: ChobitAniValueError) -> Self {
        Error::ChobitAniValueError(error)
    }
}

impl From<YamlError> for Error {
    #[inline]
    fn from(error: YamlError) -> Self {
        Error::YamlError(error)
    }
}

impl From<LoadError> for Error {
    #[inline]
    fn from(error: LoadError) -> Self {
        Error::LoadError(error)
    }
}

impl From<ApplicationError> for Error {
    #[inline]
    fn from(error: ApplicationError) -> Self {
        Error::ApplicationError(error)
    }
}

impl From<std::cell::BorrowMutError> for Error {
    #[inline]
    fn from(error: std::cell::BorrowMutError) -> Self {
        Error::BorrowMutError(error)
    }
}

impl Error {
    pub fn error_log(&self) {
        match self {
            Error::IOError(error) => {
                error!(
                    r#"{{"error": IOError, "message": "{}"}}"#,
                    error.to_string()
                );
            },

            Error::ImageError(error) => {
                error!(
                    r#"{{"error": ImageError, "message": "{}"}}"#,
                    error.to_string()
                );
            },

            Error::ChobitMapError(error) => {
                error!("{}", error);
            },

            Error::ChobitAniValueError(error) => {
                error!("{}", error);
            },

            Error::YamlError(error) => {
                error!(
                    r#"{{"error": YamlError, "message": "{}"}}"#,
                    error.to_string()
                );
            },

            Error::NoSkin {id} => {
                error!(
                    r#"{{"error": NoSkin, "id", {}}}"#,
                    id
                );
            },

            Error::LoadError(error) => {
                error!("{}", error);
            },

            Error::ApplicationError(error) => {
                error!("{}", error);
            },

            Error::BorrowMutError(error) => {
                error!("BorrowMutError: {}", error);
            }
        }
    }
}
