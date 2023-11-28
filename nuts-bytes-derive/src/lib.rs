// MIT License
//
// Copyright (c) 2023 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

/// Derive macro implementation of the [`FromBytes`] trait.
///
/// This derive macro generates a [`FromBytes`] implementation for `struct` and
/// `enum` types. `union` types are currently not supported.
///
/// [`FromBytes`]: trait.FromBytes.html
#[proc_macro_derive(FromBytes)]
pub fn from_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let from_impl = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let field_name = &field.ident;

                    quote!(
                        #field_name: FromBytes::from_bytes(source)?
                    )
                });

                quote!( Ok(#name { #(#fields,)* }) )
            }
            Fields::Unnamed(fields) => {
                let fields =
                    (0..fields.unnamed.len()).map(|_| quote!(FromBytes::from_bytes(source)?));

                quote!(
                    Ok(#name( #(#fields,)* ))
                )
            }
            Fields::Unit => quote!(
                Ok(#name)
            ),
        },
        Data::Enum(data) => {
            if data.variants.len() > 0 {
                let variants = data.variants.iter().enumerate().map(|(idx, variant)| {
                    let variant_name = &variant.ident;

                    let fields = match &variant.fields {
                        Fields::Named(fields) => {
                            let fields = fields.named.iter().map(|field| {
                                let field_name = &field.ident;

                                quote!(
                                    #field_name: FromBytes::from_bytes(source)?
                                )
                            });

                            quote!( { #(#fields,)* } )
                        }
                        Fields::Unnamed(fields) => {
                            let fields = (0..fields.unnamed.len())
                                .map(|_| quote!(FromBytes::from_bytes(source)?));

                            quote!(
                                ( #(#fields,)* )
                            )
                        }
                        Fields::Unit => quote!(),
                    };

                    quote!(
                        #idx => {
                            Ok(#name::#variant_name #fields )
                        }
                    )
                });

                quote!(
                    let idx: usize = FromBytes::from_bytes(source)?;

                    match idx {
                        #(#variants,)*
                        _=> Err(nuts_bytes::FromBytesError::InvalidVariantIndex(idx))
                    }
                )
            } else {
                let span = name.span();

                quote_spanned!(
                    span => compile_error!("zero-variant enums cannot be instantiated")
                )
            }
        }
        Data::Union(_data) => {
            let span = name.span();

            quote_spanned! {
                span => compile_error!("the union type is currently not supported")
            }
        }
    };

    let expanded = quote! {
        impl #impl_generics nuts_bytes::FromBytes for #name #ty_generics #where_clause {
            fn from_bytes<TB: nuts_bytes::TakeBytes>(source: &mut TB) -> Result<Self, nuts_bytes::FromBytesError> {
                #from_impl
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro implementation of the [`ToBytes`] trait.
///
/// This derive macro generates a [`ToBytes`] implementation for `struct` and
/// `enum` types. `union` types are currently not supported.
///
/// [`ToBytes`]: trait.ToBytes.html
#[proc_macro_derive(ToBytes)]
pub fn to_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let to_impl = match input.data {
        Data::Struct(data) => {
            let fields = data.fields.iter().enumerate().map(|(idx, field)| {
                let variant_idx = Index::from(idx);
                let field_ref = field
                    .ident
                    .as_ref()
                    .map_or_else(|| quote!(&self.#variant_idx), |ident| quote!(&self.#ident));

                quote!(ToBytes::to_bytes(#field_ref, target)?)
            });

            quote! {
                let mut n = 0;

                #(n += #fields;)*

                Ok(n)
            }
        }
        Data::Enum(data) => {
            if data.variants.len() > 0 {
                let variants = data.variants.iter().enumerate().map(|(idx, variant)| {
                    let variant_idx = Index::from(idx);
                    let variant_name = &variant.ident;

                    let left_arm_args = variant.fields.iter().enumerate().map(|(idx, field)| {
                        let ident = field.ident.as_ref().map_or_else(
                            || format_ident!("f{}", Index::from(idx)),
                            |ident| ident.clone(),
                        );

                        quote!(#ident)
                    });
                    let left_arm = match &variant.fields {
                        Fields::Named(_) => {
                            quote!( #name::#variant_name { #(#left_arm_args),* } )
                        }
                        Fields::Unnamed(_) => {
                            quote!( #name::#variant_name ( #(#left_arm_args),* ) )
                        }
                        Fields::Unit => quote!( #name::#variant_name ),
                    };

                    let right_arm_fields = variant.fields.iter().enumerate().map(|(idx, field)| {
                        let ident = field.ident.as_ref().map_or_else(
                            || format_ident!("f{}", Index::from(idx)),
                            |ident| ident.clone(),
                        );
                        quote!(ToBytes::to_bytes(#ident, target)?)
                    });
                    let right_arm = quote! {
                        {
                            let mut m = 0;

                            m += ToBytes::to_bytes(&(#variant_idx as usize), target)?;
                            #(m += #right_arm_fields;)*

                            m
                        }
                    };

                    quote! {
                        #left_arm => #right_arm
                    }
                });

                quote! {
                    let n = match self {
                        #(#variants,)*
                    };

                    Ok(n)
                }
            } else {
                let span = name.span();

                quote_spanned!(
                    span => compile_error!("zero-variant enums cannot be instantiated")
                )
            }
        }
        Data::Union(_data) => {
            let span = name.span();

            quote_spanned! {
                span => compile_error!("the union type is currently not supported")
            }
        }
    };

    let expanded = quote! {
        impl #impl_generics nuts_bytes::ToBytes for #name #ty_generics #where_clause {
            fn to_bytes<PB: nuts_bytes::PutBytes>(&self, target: &mut PB) -> Result<usize, nuts_bytes::ToBytesError> {
                #to_impl
            }
        }
    };

    TokenStream::from(expanded)
}
