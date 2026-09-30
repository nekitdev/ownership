use syn::{GenericParam, Generics};

pub fn remove_one(parameter: &mut GenericParam) {
    match parameter {
        GenericParam::Lifetime(_) => {
            // lifetime parameters do not have defaults, so nothing to do here
        }
        GenericParam::Type(type_parameter) => {
            type_parameter.default = None;
        }
        GenericParam::Const(const_parameter) => {
            const_parameter.default = None;
        }
    }
}

pub fn remove(generics: &mut Generics) {
    generics.params.iter_mut().for_each(remove_one);
}
