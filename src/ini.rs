#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use libc_print::std_name::*;
use {
    crate::binds,
    ahash::AHasher,
    alloc::{boxed::Box, ffi::CString, string::String, vec::Vec},
    core::{
        error::Error,
        ffi::{c_char, c_int, c_void, CStr},
        hash::BuildHasherDefault,
        ops::Deref,
        str::FromStr
    },
    indexmap::IndexMap
};

pub type IniMap = IndexMap<Box<str>, Option<Box<str>>, BuildHasherDefault<AHasher>>;

#[derive(Debug, Clone)]
pub struct Ini {
    items: IniMap
}

impl Deref for Ini {
    type Target = IniMap;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<'a> IntoIterator for &'a Ini {
    type Item = (&'a str, Option<&'a str>);

    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref().map(|v| v.as_ref())))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl Ini {
    pub fn from_file(path: &dyn AsRef<str>) -> Result<Self, Box<dyn Error>> {
        let mut this = Self { items: Default::default() };

        let c_path = &[path.as_ref().as_bytes(), b"\0"].concat();
        let c_path = CStr::from_bytes_with_nul(c_path)?;

        unsafe {
            if libc::access(c_path.as_ptr().cast(), libc::F_OK) != 0 {
                Err(["Config file not found: ", path.as_ref()].concat())?;
            }

            if binds::ini_parse(
                c_path.as_ptr().cast(),
                Some(Self::ini_parse_callback),
                (&mut this.items as *mut IniMap).cast()
            ) != 0
            {
                Err(["Couldn't parse INI file: ", path.as_ref()].concat())?;
            }
        }

        Ok(this)
    }

    pub fn setenv(&self, overwrite: bool) -> Result<&Self, Box<dyn Error>> {
        for (k, v) in self.iter() {
            if let Some(v) = v {
                let name = CString::from_str(k.as_ref())?;
                let value = CString::from_str(v.as_ref())?;
                unsafe {
                    libc::setenv(
                        name.as_ptr().cast(),
                        value.as_ptr().cast(),
                        overwrite.into()
                    );
                }
            }
        }

        Ok(self)
    }

    pub fn dotenv(overwrite: bool) -> Result<Self, Box<dyn Error>> {
        let ini = Self::from_file(&".env")?;
        ini.setenv(overwrite)?;

        Ok(ini)
    }

    unsafe extern "C" fn ini_parse_callback(
        user: *mut c_void,
        section: *const c_char,
        name: *const c_char,
        value: *const c_char
    ) -> c_int {
        let items: &mut IniMap = &mut *user.cast();
        let section = CStr::from_ptr(section);
        let name = CStr::from_ptr(name);
        let value = CStr::from_ptr(value);

        let key: String = if section.is_empty() {
            name.to_string_lossy().into_owned()
        } else {
            section.to_string_lossy().into_owned() + "." + &name.to_string_lossy()
        };

        let mut value: String = value.to_string_lossy().into_owned();

        if let (Some(fc), Some(lc)) = (value.chars().next(), value.chars().last()) {
            if ['\'', '\"'].contains(&fc) && fc == lc && value.chars().count() > 1 {
                value = value.trim_matches(fc).into();
            };
        }

        items.insert(
            key.into(),
            if value.is_empty() { None } else { Some(value.into()) }
        );

        return 1;
    }
}
