// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod test {
    pub mod callee {
      
      #[allow(clippy::all)]
      pub mod api {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        #[derive(Clone)]
        pub struct CalleeType {
          pub value: wit_bindgen::rt::string::String,
        }
        impl ::core::fmt::Debug for CalleeType {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("CalleeType").field("value", &self.value).finish()
          }
        }
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "test:callee/api#run"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_run(arg0: i32,arg1: i32,) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let len0 = arg1 as usize;
            let bytes0 = Vec::from_raw_parts(arg0 as *mut _, len0, len0);
            let result1 = <_GuestImpl as Guest>::run(wit_bindgen::rt::string_lift(bytes0));
            let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
            let CalleeType{ value:value3, } = result1;
            let vec4 = (value3.into_bytes()).into_boxed_slice();
            let ptr4 = vec4.as_ptr() as i32;
            let len4 = vec4.len() as i32;
            ::core::mem::forget(vec4);
            *((ptr2 + 4) as *mut i32) = len4;
            *((ptr2 + 0) as *mut i32) = ptr4;
            ptr2
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_test:callee/api#run"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_run(arg0: i32,) {
              let l0 = *((arg0 + 0) as *const i32);
              let l1 = *((arg0 + 4) as *const i32);
              wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
            }
          };
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn run(name: wit_bindgen::rt::string::String,) -> CalleeType;
        }
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        
        #[repr(align(4))]
        struct _RetArea([u8; 8]);
        static mut _RET_AREA: _RetArea = _RetArea([0; 8]);
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:callee"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 306] = [3, 0, 6, 99, 97, 108, 108, 101, 101, 0, 97, 115, 109, 13, 0, 1, 0, 7, 72, 1, 65, 2, 1, 66, 4, 1, 114, 1, 5, 118, 97, 108, 117, 101, 115, 4, 0, 11, 99, 97, 108, 108, 101, 101, 45, 116, 121, 112, 101, 3, 0, 0, 1, 64, 1, 4, 110, 97, 109, 101, 115, 0, 1, 4, 0, 3, 114, 117, 110, 1, 2, 4, 1, 15, 116, 101, 115, 116, 58, 99, 97, 108, 108, 101, 101, 47, 97, 112, 105, 5, 0, 11, 9, 1, 0, 3, 97, 112, 105, 3, 0, 0, 7, 98, 1, 65, 2, 1, 65, 2, 1, 66, 4, 1, 114, 1, 5, 118, 97, 108, 117, 101, 115, 4, 0, 11, 99, 97, 108, 108, 101, 101, 45, 116, 121, 112, 101, 3, 0, 0, 1, 64, 1, 4, 110, 97, 109, 101, 115, 0, 1, 4, 0, 3, 114, 117, 110, 1, 2, 4, 1, 15, 116, 101, 115, 116, 58, 99, 97, 108, 108, 101, 101, 47, 97, 112, 105, 5, 0, 4, 1, 18, 116, 101, 115, 116, 58, 99, 97, 108, 108, 101, 101, 47, 99, 97, 108, 108, 101, 101, 4, 0, 11, 12, 1, 0, 6, 99, 97, 108, 108, 101, 101, 3, 2, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
