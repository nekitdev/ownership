#![allow(dead_code)] // allow `input` to be used later if needed

use syn::{
    Data as DataInput, DeriveInput, Field as FieldInput, Fields as FieldsInput, Generics, Ident,
    Member, Variant as VariantInput,
};

use crate::{
    attributes::{ContainerAttributes, FieldAttributes, VariantAttributes},
    context::Context,
    name::Name,
};

#[derive(Clone, Copy)]
pub enum Style {
    Struct,
    Tuple,
    Unit,
}

impl Style {
    pub const fn of(fields: &FieldsInput) -> Self {
        match fields {
            FieldsInput::Named(_) => Self::Struct,
            FieldsInput::Unnamed(_) => Self::Tuple,
            FieldsInput::Unit => Self::Unit,
        }
    }
}

pub struct Container<'c> {
    pub name: Ident,
    pub attributes: ContainerAttributes,
    pub data: Data<'c>,
    pub generics: &'c Generics,
    pub input: &'c DeriveInput,
}

pub type Variants<'v> = [Variant<'v>];
pub type Fields<'f> = [Field<'f>];

pub enum Data<'d> {
    Enum(Vec<Variant<'d>>),
    Struct(Style, Vec<Field<'d>>),
}

pub struct Variant<'v> {
    pub name: Ident,
    pub attributes: VariantAttributes,
    pub style: Style,
    pub fields: Vec<Field<'v>>,
    pub input: &'v VariantInput,
}

impl Variant<'_> {
    pub const fn name(&self) -> &Ident {
        &self.name
    }

    pub fn fields(&self) -> &Fields<'_> {
        &self.fields
    }
}

pub struct Field<'f> {
    pub member: Member,
    pub attributes: FieldAttributes,
    pub input: &'f FieldInput,
}

impl Field<'_> {
    pub const fn member(&self) -> &Member {
        &self.member
    }
}

impl<'c> Container<'c> {
    pub fn from_ast(context: &Context, input: &'c DeriveInput) -> Option<Self> {
        let attributes = ContainerAttributes::from_ast(context, input.attrs.iter());

        let data = match input.data {
            DataInput::Enum(ref enum_data) => {
                Data::Enum(enum_from_ast(context, enum_data.variants.iter()))
            }
            DataInput::Struct(ref struct_data) => {
                let (style, fields) = struct_from_ast(context, &struct_data.fields);

                Data::Struct(style, fields)
            }
            DataInput::Union(_) => {
                let message = format!(
                    "`{ownership}` does not support `{derive}` for unions",
                    ownership = Name::OWNERSHIP,
                    derive = Name::DERIVE
                );

                context.error_spanned_by(input, message);

                return None;
            }
        };

        let item = Self {
            name: input.ident.clone(),
            attributes,
            data,
            generics: &input.generics,
            input,
        };

        Some(item)
    }
}

fn enum_from_ast<'a, V: IntoIterator<Item = &'a VariantInput>>(
    context: &Context,
    variants: V,
) -> Vec<Variant<'a>> {
    variants
        .into_iter()
        .map(|input| {
            let attributes = VariantAttributes::from_ast(context, input.attrs.iter());

            let (style, fields) = struct_from_ast(context, &input.fields);

            let name = input.ident.clone();

            Variant {
                name,
                attributes,
                style,
                fields,
                input,
            }
        })
        .collect()
}

fn struct_from_ast<'a>(context: &Context, fields: &'a FieldsInput) -> (Style, Vec<Field<'a>>) {
    (Style::of(fields), fields_from_ast(context, fields.iter()))
}

fn fields_from_ast<'a, F: IntoIterator<Item = &'a FieldInput>>(
    context: &Context,
    fields: F,
) -> Vec<Field<'a>> {
    fields
        .into_iter()
        .enumerate()
        .map(|(index, input)| {
            let member = member(index, input.ident.as_ref());

            let attributes = FieldAttributes::from_ast(context, input.attrs.iter());

            Field {
                member,
                attributes,
                input,
            }
        })
        .collect()
}

fn member(index: usize, option: Option<&Ident>) -> Member {
    option.map_or_else(
        || Member::Unnamed(index.into()),
        |name| Member::Named(name.clone()),
    )
}
