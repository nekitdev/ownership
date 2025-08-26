use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{DeriveInput, Ident, Path, Result, parse_quote, parse_quote_spanned, spanned::Spanned};

use crate::{
    ast::{Container, Data, Field, Fields, Style, Variant, Variants},
    context::Context,
    dummy::wrap_in_const,
    parameters::Parameters,
};

pub fn derive_into_owned(input: &DeriveInput) -> Result<TokenStream> {
    let context = Context::new();

    let Some(container) = Container::from_ast(&context, input) else {
        return Err(context.check().unwrap_err());
    };

    context.check()?;

    let parameters = Parameters::new(&container);

    let name = parameters.name;

    let generics = parameters.generics;
    let generic_arguments = parameters.generic_arguments;

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let body = into_owned_body(&container);

    let into_owned = into_owned_trait();

    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics #into_owned for #name #type_generics #where_clause {
            type Owned = #name #generic_arguments;

            fn into_owned(self) -> <Self as #into_owned>::Owned {
                #body
            }
        }
    };

    Ok(wrap_in_const(&impl_block))
}

pub fn into_owned_trait() -> Path {
    parse_quote!(_ownership::IntoOwned)
}

pub fn into_owned_method(span: Span) -> Path {
    parse_quote_spanned!(span => _ownership::IntoOwned::into_owned)
}

pub fn into_owned_body(container: &Container<'_>) -> TokenStream {
    if container.attributes.as_is() {
        return into_owned_as_is();
    }

    match container.data {
        Data::Enum(ref variants) => into_owned_enum(variants),
        Data::Struct(style, ref fields) => into_owned_struct(style, fields),
    }
}

pub fn into_owned_as_is() -> TokenStream {
    quote! {
        self
    }
}

pub fn into_owned_enum(variants: &Variants<'_>) -> TokenStream {
    let arms = variants.iter().map(into_owned_variant);

    quote! {
        match self {
            #(#arms)*
        }
    }
}

pub fn into_owned_variant(variant: &Variant<'_>) -> TokenStream {
    if variant.attributes.as_is() {
        return into_owned_variant_as_is(variant);
    }

    let case = into_owned_case(variant);
    let body = into_owned_style_variant(variant);

    quote! {
        #case => #body,
    }
}

pub fn into_owned_case(variant: &Variant<'_>) -> TokenStream {
    let name = variant.name();

    match variant.style {
        Style::Unit => quote! {
            Self::#name
        },
        Style::Tuple => {
            let names = (0..variant.fields.len()).map(index_to_name);

            quote! {
                Self::#name(
                    #(#names),*
                )
            }
        }
        Style::Struct => {
            let members = variant.fields.iter().map(Field::member);

            quote! {
                Self::#name {
                    #(#members),*
                }
            }
        }
    }
}

pub fn into_owned_style_variant(variant: &Variant<'_>) -> TokenStream {
    match variant.style {
        Style::Unit => into_owned_as_is(),
        Style::Tuple => into_owned_tuple_variant(variant.name(), variant.fields()),
        Style::Struct => into_owned_struct_variant(variant.name(), variant.fields()),
    }
}

pub fn into_owned_tuple_variant(name: &Ident, fields: &Fields<'_>) -> TokenStream {
    let generated = fields.iter().enumerate().map(|(index, field)| {
        let name = index_to_name(index);

        if field.attributes.as_is() {
            name.into_token_stream()
        } else {
            let into_owned = into_owned_method(field.input.span());

            quote! {
                #into_owned(#name)
            }
        }
    });

    quote! {
        Self::Owned::#name(
            #(#generated),*
        )
    }
}

pub fn into_owned_struct_variant(name: &Ident, fields: &Fields<'_>) -> TokenStream {
    let generated = fields.iter().map(|field| {
        let member = field.member();

        if field.attributes.as_is() {
            member.to_token_stream()
        } else {
            let into_owned = into_owned_method(field.input.span());

            quote! {
                #member: #into_owned(#member)
            }
        }
    });

    quote! {
        Self::Owned::#name {
            #(#generated),*
        }
    }
}

pub fn into_owned_variant_as_is(variant: &Variant<'_>) -> TokenStream {
    let name = variant.name();

    let pattern = match variant.style {
        Style::Unit => quote!(),
        Style::Tuple => quote!((..)),
        Style::Struct => quote!({ .. }),
    };

    let as_is = into_owned_as_is();

    quote! {
        Self::#name #pattern => #as_is,
    }
}

pub fn into_owned_struct(style: Style, fields: &Fields<'_>) -> TokenStream {
    match style {
        Style::Struct => into_owned_actual_struct(fields),
        Style::Tuple => into_owned_tuple_struct(fields),
        Style::Unit => into_owned_as_is(),
    }
}

pub fn into_owned_actual_struct(fields: &Fields<'_>) -> TokenStream {
    let generated = fields.iter().map(|field| {
        let member = field.member();

        if field.attributes.as_is() {
            quote! {
                #member: self.#member
            }
        } else {
            let into_owned = into_owned_method(field.input.span());

            quote! {
                #member: #into_owned(self.#member)
            }
        }
    });

    quote! {
        Self::Owned {
            #(#generated),*
        }
    }
}

pub fn into_owned_tuple_struct(fields: &Fields<'_>) -> TokenStream {
    let generated = fields.iter().map(|field| {
        let member = field.member();

        if field.attributes.as_is() {
            quote! {
                self.#member
            }
        } else {
            let into_owned = into_owned_method(field.input.span());

            quote! {
                #into_owned(self.#member)
            }
        }
    });

    quote! {
        Self::Owned(
            #(#generated),*
        )
    }
}

pub fn index_to_name(index: usize) -> Ident {
    let string = format!("index_{index}");

    Ident::new(string.as_str(), Span::call_site())
}
