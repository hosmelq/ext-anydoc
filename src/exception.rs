use ext_php_rs::{
    class::RegisteredClass,
    convert::IntoZval,
    exception::PhpException,
    ffi::{zend_class_entry, zend_object},
    flags::ClassFlags,
    prelude::*,
    zend::ce,
};

unsafe extern "C" {
    fn zend_update_property_stringl(
        scope: *const zend_class_entry,
        object: *mut zend_object,
        name: *const std::ffi::c_char,
        name_length: usize,
        value: *const std::ffi::c_char,
        value_length: usize,
    );
}

#[php_class]
#[php(name = "Anydoc\\Exception\\ConvertException")]
#[php(extends(ce = ce::exception, stub = "\\Exception"))]
#[derive(Default)]
pub struct ConvertException;

#[php_class]
#[php(name = "Anydoc\\Exception\\UnsupportedException")]
#[php(extends(ConvertException))]
#[php(flags = ClassFlags::Final)]
#[derive(Default)]
pub struct UnsupportedException {
    #[php(prop)]
    pub detail: String,
}

#[php_impl]
impl UnsupportedException {
    const ERROR_CODE: &'static str = "unsupported";
}

#[php_class]
#[php(name = "Anydoc\\Exception\\MalformedException")]
#[php(extends(ConvertException))]
#[php(flags = ClassFlags::Final)]
#[derive(Default)]
pub struct MalformedException {
    #[php(prop)]
    pub part: Option<String>,
    #[php(prop)]
    pub detail: String,
}

#[php_impl]
impl MalformedException {
    const ERROR_CODE: &'static str = "malformed";
}

#[php_class]
#[php(name = "Anydoc\\Exception\\EncryptedException")]
#[php(extends(ConvertException))]
#[php(flags = ClassFlags::Final)]
#[derive(Default)]
pub struct EncryptedException;

#[php_impl]
impl EncryptedException {
    const ERROR_CODE: &'static str = "encrypted";
}

#[php_class]
#[php(name = "Anydoc\\Exception\\ResourceLimitException")]
#[php(extends(ConvertException))]
#[php(flags = ClassFlags::Final)]
#[derive(Default)]
pub struct ResourceLimitException {
    #[php(prop)]
    pub limit: String,
    #[php(prop)]
    pub detail: String,
}

#[php_impl]
impl ResourceLimitException {
    const ERROR_CODE: &'static str = "resourceLimit";
}

#[php_class]
#[php(name = "Anydoc\\Exception\\MissingPartException")]
#[php(extends(ConvertException))]
#[php(flags = ClassFlags::Final)]
#[derive(Default)]
pub struct MissingPartException {
    #[php(prop)]
    pub part: String,
}

#[php_impl]
impl MissingPartException {
    const ERROR_CODE: &'static str = "missingPart";
}

#[php_class]
#[php(name = "Anydoc\\Exception\\IoException")]
#[php(extends(ConvertException))]
#[php(flags = ClassFlags::Final)]
#[derive(Default)]
pub struct IoException;

#[php_impl]
impl IoException {
    const ERROR_CODE: &'static str = "io";
}

#[php_class]
#[php(name = "Anydoc\\Exception\\PanicException")]
#[php(extends(ce = ce::exception, stub = "\\Exception"))]
#[php(flags = ClassFlags::Final)]
#[derive(Default)]
pub struct PanicException;

pub fn convert(error: anydoc::ConvertError) -> PhpException {
    let message = error.to_string();

    match error {
        anydoc::ConvertError::Unsupported(detail) => {
            with_object(UnsupportedException { detail }, message)
        }
        anydoc::ConvertError::Malformed { part, detail } => {
            with_object(MalformedException { part, detail }, message)
        }
        anydoc::ConvertError::Encrypted => PhpException::from_class::<EncryptedException>(message),
        anydoc::ConvertError::ResourceLimit { limit, detail } => with_object(
            ResourceLimitException {
                limit: limit.into(),
                detail,
            },
            message,
        ),
        anydoc::ConvertError::MissingPart { part } => {
            with_object(MissingPartException { part }, message)
        }
        anydoc::ConvertError::Io(_) => PhpException::from_class::<IoException>(message),
        _ => PhpException::from_class::<ConvertException>(message),
    }
}

fn with_object<T>(value: T, message: String) -> PhpException
where
    T: IntoZval + RegisteredClass,
{
    let Ok(mut object) = value.into_zval(false) else {
        return PhpException::from_class::<T>(message);
    };
    let Some(zend_object) = object.object_mut() else {
        return PhpException::from_class::<T>(message);
    };

    // Direct Rust construction bypasses Exception::__construct(). Set the
    // inherited protected property with Exception as the Zend scope.
    unsafe {
        zend_update_property_stringl(
            ce::exception(),
            zend_object,
            b"message".as_ptr().cast(),
            "message".len(),
            message.as_ptr().cast(),
            message.len(),
        );
    }

    PhpException::from_class::<T>(message).with_object(object)
}

pub fn panic() -> PhpException {
    PhpException::from_class::<PanicException>(
        "anydoc panicked while processing the document".into(),
    )
}
