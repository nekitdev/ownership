use syn::{
    Data as DataInput, DeriveInput, Field as FieldInput, Fields as FieldsInput, Generics, Ident,
    Index, Member, Variant as VariantInput,
};

use crate::{
    By,
    attributes::{ContainerAttributes, FieldAttributes, VariantAttributes},
    context::Context,
    index::{call_site, indexer},
    names::Name,
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
    // pub input: &'c DeriveInput, // unused
}

impl<'c> Container<'c> {
    pub const fn attributes(&self) -> ContainerAttributes {
        self.attributes
    }

    pub const fn data(&self) -> &Data<'c> {
        &self.data
    }

    pub const fn generics(&self) -> &'c Generics {
        self.generics
    }
}

pub type Variants<'v> = [Variant<'v>];
pub type Fields<'f> = [Field<'f>];

pub struct EnumData<'e> {
    pub variants: Vec<Variant<'e>>,
}

impl<'e> EnumData<'e> {
    pub const fn new(variants: Vec<Variant<'e>>) -> Self {
        Self { variants }
    }

    pub const fn variants(&self) -> &Variants<'e> {
        self.variants.as_slice()
    }

    pub fn from_variants_ast<V: IntoIterator<Item = &'e VariantInput>>(
        context: &Context,
        variants: V,
    ) -> Self {
        let collected = variants
            .into_iter()
            .map(|input| {
                let attributes = VariantAttributes::from_ast(context, input.attrs.iter());

                let data = StructData::from_ast(context, input.fields.by_ref());

                let name = input.ident.clone();

                Variant {
                    name,
                    attributes,
                    data,
                    // input,
                }
            })
            .collect();

        Self::new(collected)
    }
}

pub struct StructData<'s> {
    pub style: Style,
    pub fields: Vec<Field<'s>>,
}

impl<'s> StructData<'s> {
    pub const fn new(style: Style, fields: Vec<Field<'s>>) -> Self {
        Self { style, fields }
    }

    pub const fn style(&self) -> Style {
        self.style
    }

    pub const fn fields(&self) -> &Fields<'s> {
        self.fields.as_slice()
    }

    pub fn from_ast(context: &Context, fields: &'s FieldsInput) -> Self {
        let style = Style::of(fields);

        Self::from_style_and_fields_ast(context, style, fields.iter())
    }

    pub fn from_style_and_fields_ast<F: IntoIterator<Item = &'s FieldInput>>(
        context: &Context,
        style: Style,
        fields: F,
    ) -> Self {
        let collected = indexer()
            .map(call_site)
            .zip(fields)
            .map(|(index, input)| {
                let member = member(index, input.ident.clone());

                let attributes = FieldAttributes::from_ast(context, input.attrs.iter());

                Field {
                    member,
                    attributes,
                    input,
                }
            })
            .collect();

        Self::new(style, collected)
    }
}

pub enum Data<'d> {
    Enum(EnumData<'d>),
    Struct(StructData<'d>),
}

impl<'d> Data<'d> {
    pub fn from_ast(context: &Context, input: &'d DataInput) -> Option<Self> {
        match input {
            DataInput::Enum(enum_data) => {
                let data = EnumData::from_variants_ast(context, enum_data.variants.iter());

                Some(Self::Enum(data))
            }
            DataInput::Struct(struct_data) => {
                let data = StructData::from_ast(context, struct_data.fields.by_ref());

                Some(Self::Struct(data))
            }
            DataInput::Union(_) => None,
        }
    }
}

pub struct Variant<'v> {
    pub name: Ident,
    pub attributes: VariantAttributes,
    pub data: StructData<'v>,
    // pub input: &'v VariantInput, // unused
}

impl Variant<'_> {
    pub const fn name(&self) -> &Ident {
        &self.name
    }

    pub const fn attributes(&self) -> VariantAttributes {
        self.attributes
    }

    pub const fn style(&self) -> Style {
        self.data.style()
    }

    pub const fn fields(&self) -> &Fields<'_> {
        self.data.fields()
    }
}

pub struct Field<'f> {
    pub member: Member,
    pub attributes: FieldAttributes,
    pub input: &'f FieldInput,
}

impl<'f> Field<'f> {
    pub const fn member(&self) -> &Member {
        &self.member
    }

    pub const fn attributes(&self) -> FieldAttributes {
        self.attributes
    }

    pub const fn input(&self) -> &'f FieldInput {
        self.input
    }
}

impl<'c> Container<'c> {
    pub fn from_ast(context: &Context, input: &'c DeriveInput) -> Option<Self> {
        let attributes = ContainerAttributes::from_ast(context, input.attrs.iter());

        let data = match Data::from_ast(context, input.data.by_ref()) {
            Some(data) => data,
            None => {
                let message = format!(
                    "`{ownership}` does not support `{derive}` for unions",
                    ownership = Name::OWNERSHIP,
                    derive = Name::DERIVE
                );

                context.error_spanned_by(input, message);

                return None;
            }
        };

        let name = input.ident.clone();
        let generics = input.generics.by_ref();

        let item = Self {
            name,
            attributes,
            data,
            generics,
            // input,
        };

        Some(item)
    }
}

pub fn member(index: Index, maybe: Option<Ident>) -> Member {
    maybe.map_or_else(|| Member::Unnamed(index), Member::Named)
}
