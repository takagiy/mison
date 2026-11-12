//! `#[derive(Encode, Decode)]` for mison: a struct with named fields is a record, and an enum is a variant.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::punctuated::Punctuated;
use syn::{Data, DeriveInput, Error, Fields, FieldsNamed, Ident, Token, Variant, parse_macro_input, parse_quote};

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    encode(parse_macro_input!(input as DeriveInput)).unwrap_or_else(Error::into_compile_error).into()
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    decode(parse_macro_input!(input as DeriveInput)).unwrap_or_else(Error::into_compile_error).into()
}

/// The types the derives take: a struct with named fields, or an enum with variants.
enum Shape<'a> {
    Struct(&'a FieldsNamed),
    Enum(&'a Punctuated<Variant, Token![,]>),
}

fn shape(input: &DeriveInput) -> syn::Result<Shape<'_>> {
    let name = &input.ident;
    match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) if !fields.named.is_empty() => Ok(Shape::Struct(fields)),
            _ => Err(Error::new_spanned(name, "mison needs a struct with named fields")),
        },
        Data::Enum(e) if !e.variants.is_empty() => Ok(Shape::Enum(&e.variants)),
        Data::Enum(_) => Err(Error::new_spanned(name, "mison needs an enum with variants")),
        Data::Union(_) => Err(Error::new_spanned(name, "mison does not support unions")),
    }
}

fn encode(input: DeriveInput) -> syn::Result<TokenStream> {
    let write = match shape(&input)? {
        Shape::Struct(fields) => {
            let write = write_fields(fields, |f| quote!(&self.#f));
            quote!(w.record(|r| { #write });)
        }
        Shape::Enum(variants) => {
            let arms = variants.iter().map(|v| {
                let (ident, label) = (&v.ident, v.ident.unraw().to_string());
                match &v.fields {
                    Fields::Unit => quote!(Self::#ident => w.unit_variant(#label)),
                    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        quote!(Self::#ident(x) => w.variant(#label, |w| ::mison::Encode::write(x, w)))
                    }
                    Fields::Unnamed(fields) => {
                        let vars: Vec<_> = (0..fields.unnamed.len()).map(|i| format_ident!("x{i}")).collect();
                        quote!(Self::#ident(#(#vars),*) => w.variant(#label, |w| ::mison::Encode::write(&(#(#vars,)*), w)))
                    }
                    Fields::Named(fields) => {
                        let idents = fields.named.iter().map(|f| &f.ident);
                        let vars = fields.named.iter().map(|f| var(f.ident.as_ref().unwrap()));
                        let write = write_fields(fields, |f| { let v = var(f); quote!(#v) });
                        quote!(Self::#ident { #(#idents: #vars),* } => w.variant(#label, |w| w.record(|r| { #write })))
                    }
                }
            });
            quote!(match self { #(#arms,)* })
        }
    };
    Ok(implement(
        input,
        quote!(::mison::Encode),
        quote! {
            const HEIGHT: usize = 0;

            fn write(&self, w: &mut ::mison::Writer) {
                #write
            }
        },
    ))
}

fn decode(input: DeriveInput) -> syn::Result<TokenStream> {
    let read = match shape(&input)? {
        Shape::Struct(fields) => read_fields(quote!(Self), fields),
        Shape::Enum(variants) => {
            let arms = variants.iter().map(|v| {
                let (ident, label) = (&v.ident, v.ident.unraw().to_string());
                match &v.fields {
                    Fields::Unit => quote!(#label => Ok(Self::#ident)),
                    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        let ty = &fields.unnamed[0].ty;
                        quote!(#label => r.contents(|r| Ok(Self::#ident(<#ty as ::mison::Decode>::read(r)?))))
                    }
                    Fields::Unnamed(fields) => {
                        let vars: Vec<_> = (0..fields.unnamed.len()).map(|i| format_ident!("x{i}")).collect();
                        let tys = fields.unnamed.iter().map(|f| &f.ty);
                        quote!(#label => r.contents(|r| {
                            let (#(#vars,)*) = <(#(#tys,)*) as ::mison::Decode>::read(r)?;
                            Ok(Self::#ident(#(#vars),*))
                        }))
                    }
                    Fields::Named(fields) => {
                        let read = read_fields(quote!(Self::#ident), fields);
                        quote!(#label => r.contents(|r| { #read }))
                    }
                }
            });
            quote! {
                let label = r.label()?;
                match &*label {
                    #(#arms,)*
                    _ => Err(r.error(format_args!("unknown case `{}`", label))),
                }
            }
        }
    };
    Ok(implement(
        input,
        quote!(::mison::Decode),
        quote! {
            fn read(r: &mut ::mison::Reader) -> ::core::result::Result<Self, ::mison::Error> {
                #read
            }
        },
    ))
}

/// `impl trait for input { body }`, with `trait` as a bound on each type parameter.
fn implement(mut input: DeriveInput, trait_: TokenStream, body: TokenStream) -> TokenStream {
    for param in input.generics.type_params_mut() {
        param.bounds.push(parse_quote!(#trait_));
    }
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote!(impl #impl_generics #trait_ for #name #ty_generics #where_clause { #body })
}

/// The variable a field of an enum variant is bound to, apart from the names the derived code uses.
fn var(field: &Ident) -> Ident {
    format_ident!("field_{}", field.unraw())
}

/// Writes each field to the record `r`, the value taken by `value` from the field's name.
fn write_fields(fields: &FieldsNamed, value: impl Fn(&Ident) -> TokenStream) -> TokenStream {
    let calls = fields.named.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let (key, value) = (ident.unraw().to_string(), value(ident));
        quote!(r.field(#key, #value);)
    });
    quote!(#(#calls)*)
}

/// Reads a record into `path { … }`, requiring each field's key exactly once.
fn read_fields(path: TokenStream, fields: &FieldsNamed) -> TokenStream {
    let idents: Vec<_> = fields.named.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let keys: Vec<_> = idents.iter().map(|i| i.unraw().to_string()).collect();
    let vars: Vec<_> = idents.iter().map(|i| var(i)).collect();
    let tys = fields.named.iter().map(|f| &f.ty);
    quote! {
        #(let mut #vars = ::core::option::Option::None;)*
        r.record(|key, r| {
            match &*key {
                #(#keys => {
                    if #vars.is_some() {
                        return Err(r.error(format_args!("key `{}` written twice", key)));
                    }
                    #vars = Some(<#tys as ::mison::Decode>::read(r)?);
                })*
                _ => return Err(r.error(format_args!("unknown key `{}`", key))),
            }
            Ok(())
        })?;
        Ok(#path {
            #(#idents: #vars.ok_or_else(|| r.error(concat!("missing key `", #keys, "`")))?,)*
        })
    }
}
