use std::ffi::OsString;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NASError {
    #[error("Unable to initialize NAS DB")]
    DBInitializationError,

    #[error("Unable to initialize NAS AppState")]
    AppStateInitializationError,

    #[error("Invalid credentials for {username:?}")]
    UserValidationError { username: String },

    #[error("Unable to read User from DB")]
    UserReadError,

    #[error("Unable to resolve FS Root for user {username:?} from {fs_root:?}")]
    FSRootResolutionError { fs_root: String, username: String },

    #[error("The PathBuf {pathbuf:?} is invalid")]
    InvalidPathBuf { pathbuf: PathBuf },

    #[error("You do not have permissions to access {pathbuf:?}")]
    PathAccessDenied { pathbuf: PathBuf },

    #[error("Unable to get file name for {pathbuf:?}")]
    FileNameError { pathbuf: PathBuf },

    #[error("Unable to compute extension for {pathbuf:?}")]
    FileExtensionError { pathbuf: PathBuf },

    #[error("Unable to compute size for {pathbuf:?}")]
    FileSizeError { pathbuf: PathBuf },

    #[error("Cannot convert {osstring:?} to valid str")]
    OsStrConversionError { osstring: OsString },

    #[error("The path {path:?} does not exist")]
    NonExistentPath { path: String },

    #[error("Failed to render {template:?} template")]
    TemplateRenderError { template: &'static str },

    #[error("Unable to read file or directory at path {path:?}")]
    PathReadError { path: String },

    #[error("Unable to calculate breadcrumbs for {pathbuf:?}")]
    BreadcrumbError { pathbuf: PathBuf },

    #[error("Unable to create file / directory at path {path:?}")]
    PathCreateError { path: String },

    #[error("Unable to create file / directory at path {pathbuf:?} because a file already exists at this path")]
    PathExistsError { pathbuf: PathBuf },

    #[error("Unable to rename file / directory at path {pathbuf:?}")]
    PathRenameError { pathbuf: PathBuf },

    #[error("Unable to delete file / directory at path {pathbuf:?}")]
    PathDeleteError { pathbuf: PathBuf },
}

impl actix_web::error::ResponseError for NASError {}
