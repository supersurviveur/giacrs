use proc_macro::TokenStream;
use quote::quote;

/// Wrap a FFI call to giac to catch errors
#[proc_macro]
pub fn ffi_safe_call(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    TokenStream::from(quote! {
        let result = Self(unsafe { ffi::giacrs_gen_allocate() });
        let error = unsafe { #input };
        if error == std::ptr::null() {
            Ok(result)
        } else {
            Err(GiacError::InternalError(error.into()))
        }
    })
}

/// Wrap a FFI call to giac to catch errors, panic if any, edit in place
#[proc_macro]
pub fn ffi_safe_panic_inplace_call(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    TokenStream::from(quote! {
        let error = unsafe { #input };
        if error != std::ptr::null() {
            panic!("{:?}", GiacError::InternalError(error.into()));
        }
    })
}
