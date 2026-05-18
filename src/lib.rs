use proc_macro::TokenStream;
use quote::quote;
use syn::*;

/// This attribute allows you to inherit from another struct.
#[proc_macro_attribute]
pub fn inherits(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let args = parse_macro_input!(args as Path);

    let name = &ast.ident;

    if let Data::Struct(DataStruct {
        fields: Fields::Named(ref mut fields),
        ..
    }) = ast.data
    {
        let new_field: Field = parse_quote! {
            parent: #args
        };
        fields.named.push(new_field);

        let result = quote! {
            #ast

            impl std::ops::Deref for #name {
                type Target = #args;

                fn deref(&self) -> &Self::Target {
                    &self.parent
                }
            }

            impl std::ops::DerefMut for #name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.parent
                }
            }

            impl From<#name> for #args {
                fn from(value: #name) -> Self {
                    value.parent
                }
            }

            impl<'a> From<&'a #name> for &'a #args {
                fn from(value: &'a #name) -> Self {
                    &value.parent
                }
            }

            impl<'a> From<&'a mut #name> for &'a mut #args {
                fn from(value: &'a mut #name) -> Self {
                    &mut value.parent
                }
            }
        };

        TokenStream::from(result)
    } else {
        panic!("Only structs with named fields are supported")
    }
}

#[proc_macro_attribute]
pub fn new_type(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    if let Data::Struct(DataStruct {
        fields: Fields::Unnamed(ref mut fields),
        ..
    }) = ast.data
    {
        if fields.unnamed.len() != 1 {
            panic!("The #[new_type] has and can only have one field")
        }
        let ty = fields.unnamed.first().unwrap().ty.clone();
        let impl_block = quote! {
            #ast

            impl std::ops::Deref for #name {
                type Target = #ty;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl std::ops::DerefMut for #name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        };
        TokenStream::from(impl_block)
    } else {
        panic!("The #[new_type] can only be used for tuple structs")
    }
}
