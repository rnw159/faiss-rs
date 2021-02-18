use std::ffi;
use std::ptr;

use super::*;

use crate::error::Result;
use crate::faiss_try;

/// Uses a-priori knowledge on the Faiss indexes to extract tunable parameters.
pub struct ParameterSpace {
    inner: *mut FaissParameterSpace,
}

impl ParameterSpace {
    /// Create a new instance
    pub fn new() -> Result<Self> {
        unsafe {
            let mut inner = ptr::null_mut();
            faiss_try(faiss_ParameterSpace_new(&mut inner))?;

            Ok(ParameterSpace { inner })
        }
    }

    /// Set one of the parameters
    pub fn set_index_parameter(
        &self,
        index: &mut IndexImpl,
        name: &ffi::CStr,
        value: f64,
    ) -> Result<()> {
        unsafe {
            let index_ptr = index.inner_ptr();
            faiss_try(faiss_ParameterSpace_set_index_parameter(
                self.inner,
                index_ptr,
                name.as_ptr(),
                value,
            ))?;

            Ok(())
        }
    }

    /// Print a description on stdout
    pub fn display(&self) {
        unsafe {
            faiss_ParameterSpace_display(self.inner);
        }
    }

    /// nb of combinations, = product of values sizes
    pub fn n_combinations(&self) -> usize {
        unsafe { faiss_ParameterSpace_n_combinations(self.inner) }
    }
}

// impl Drop for ParameterSpace {
//     fn drop(&mut self) {
//         unsafe {
//             faiss_ParameterSpace_free(self.inner);
//         }
//     }
// } // TODO: only version >= 1.6.4

#[cfg(test)]
mod tests {
    use std::ffi;

    use super::*;
    use crate::index::index_factory;
    use crate::metric::MetricType;

    #[test]
    fn set_nprobe_in_flat() {
        let mut index = index_factory(64, "IVF8,Flat", MetricType::L2).unwrap();

        let ps = ParameterSpace::new().unwrap();
        let name = ffi::CString::new("nprobe").expect("CString::new failed");

        ps.set_index_parameter(&mut index, &name, 5.0).unwrap();

        let actual_nprobe = unsafe {
            let index_ivf = faiss_sys::faiss_IndexIVF_cast(index.inner_ptr());

            faiss_sys::faiss_IndexIVF_nprobe(index_ivf)
        };

        assert_eq!(actual_nprobe, 5usize);
    }
}
