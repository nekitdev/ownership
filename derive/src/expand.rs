use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    DeriveInput, Ident, Member, Path, Result, parse_quote, parse_quote_spanned, spanned::Spanned,
};

use crate::{
    By,
    ast::{Container, Data, EnumData, Field, Fields, StructData, Style, Variant},
    context::Context,
    dummy::wrap_in_const,
    parameters::Parameters,
};

pub const ZERO: usize = 0;
pub const NO_ERRORS: &str = "container failed to parse but no errors produced";

pub fn derive_into_owned(input: &DeriveInput) -> Result<TokenStream> {
    let context = Context::new();

    let Some(container) = Container::from_ast(context.by_ref(), input) else {
        return Err(context.check().expect_err(NO_ERRORS));
    };

    context.check()?;

    let parameters = Parameters::new(container.by_ref());

    let name = parameters.name();

    let generics = parameters.generics();
    let generic_arguments = parameters.generic_arguments();

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let body = into_owned_body(container.by_ref());

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

    let wrapped = wrap_in_const(impl_block.by_ref());

    Ok(wrapped)
}

pub fn into_owned_trait() -> Path {
    parse_quote!(_ownership::IntoOwned)
}

pub fn into_owned_method(span: Span) -> Path {
    parse_quote_spanned!(span => _ownership::IntoOwned::into_owned)
}

pub fn into_owned_body(container: &Container<'_>) -> TokenStream {
    if container.attributes().as_is() {
        return into_owned_as_is();
    }

    match container.data() {
        Data::Enum(data) => into_owned_enum(data),
        Data::Struct(data) => into_owned_struct(data),
    }
}

pub fn into_owned_as_is() -> TokenStream {
    quote! {
        self
    }
}

pub fn into_owned_enum(data: &EnumData<'_>) -> TokenStream {
    let arms = data.variants().iter().map(into_owned_variant);

    quote! {
        match self {
            #(#arms)*
        }
    }
}

pub fn into_owned_variant(variant: &Variant<'_>) -> TokenStream {
    let case = into_owned_case(variant);
    let body = into_owned_style_variant(variant);

    quote! {
        #case => #body,
    }
}

pub fn into_owned_case(variant: &Variant<'_>) -> TokenStream {
    let name = variant.name();

    match variant.style() {
        Style::Unit => quote! {
            Self::#name
        },
        Style::Tuple => {
            let names = (0..variant.fields().len()).map(index_to_name);

            quote! {
                Self::#name(
                    #(#names),*
                )
            }
        }
        Style::Struct => {
            let members = variant.fields().iter().map(member);

            quote! {
                Self::#name {
                    #(#members),*
                }
            }
        }
    }
}

pub fn into_owned_style_variant(variant: &Variant<'_>) -> TokenStream {
    let name = variant.name();
    let fields = variant.fields();
    let as_is = variant.attributes().as_is();

    match variant.style() {
        Style::Unit => into_owned_unit_variant(name),
        Style::Tuple => into_owned_tuple_variant_match(name, fields, as_is),
        Style::Struct => into_owned_struct_variant_match(name, fields, as_is),
    }
}

pub fn into_owned_unit_variant(name: &Ident) -> TokenStream {
    quote! {
        Self::Owned::#name
    }
}

pub fn into_owned_tuple_variant_match(
    name: &Ident,
    fields: &Fields<'_>,
    as_is: bool,
) -> TokenStream {
    if as_is {
        into_owned_tuple_variant_as_is(name, fields.len())
    } else {
        into_owned_tuple_variant(name, fields)
    }
}

pub fn into_owned_tuple_variant_as_is(name: &Ident, count: usize) -> TokenStream {
    let generated = (ZERO..count).map(index_to_name);

    quote! {
        Self::Owned::#name(
            #(#generated),*
        )
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

pub fn into_owned_struct_variant_match(
    name: &Ident,
    fields: &Fields<'_>,
    as_is: bool,
) -> TokenStream {
    if as_is {
        into_owned_struct_variant_as_is(name, fields)
    } else {
        into_owned_struct_variant(name, fields)
    }
}

pub fn into_owned_struct_variant_as_is(name: &Ident, fields: &Fields<'_>) -> TokenStream {
    let generated = fields.iter().map(member);

    quote! {
        Self::Owned::#name {
            #(#generated),*
        }
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

pub fn into_owned_struct(data: &StructData<'_>) -> TokenStream {
    match data.style() {
        Style::Struct => into_owned_actual_struct(data.fields()),
        Style::Tuple => into_owned_tuple_struct(data.fields()),
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

pub fn member<'f>(field: &'f Field<'_>) -> &'f Member {
    field.member()
}
