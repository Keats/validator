use darling::FromDeriveInput;
use darling::{ast::Data, util::WithOriginal};
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, GenericParam, Path, PathArguments};

use tokens::schema::schema_tokens;
use types::*;
use utils::quote_use_stmts;

mod tokens;
mod types;
mod utils;

// The main struct we get from parsing the attributes
// The "supports(struct_named)" attribute guarantees only named structs to work with this macro
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(validate), supports(struct_named))]
#[darling(and_then = ValidationData::validate)]
struct ValidationData {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), WithOriginal<ValidateField, syn::Field>>,
    #[darling(multiple)]
    schema: Vec<Schema>,
    context: Option<Path>,
    mutable: Option<bool>,
    nest_all_fields: Option<bool>,
}

impl ValidationData {
    fn validate(self) -> darling::Result<Self> {
        if let Some(context) = &self.context {
            // Check if context lifetime is not `'v_a`
            for segment in &context.segments {
                match &segment.arguments {
                    PathArguments::AngleBracketed(args) => {
                        for arg in &args.args {
                            match arg {
                                syn::GenericArgument::Lifetime(lt) => {
                                    if lt.ident != "v_a" {
                                        abort! {
                                            lt.ident, "Invalid argument reference";
                                            note = "The lifetime `'{}` is not supported.", lt.ident;
                                            help = "Please use the validator lifetime `'v_a`";
                                        }
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        match &self.data {
            Data::Struct(fields) => {
                let original_fields: Vec<&Field> =
                    fields.fields.iter().map(|f| &f.original).collect();
                for f in &fields.fields {
                    f.parsed.validate(&self.ident, &original_fields, &f.original);
                }
            }
            _ => (),
        }

        Ok(self)
    }
}

#[proc_macro_error]
#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    // parse the input to the ValidationData struct defined above
    let validation_data = match ValidationData::from_derive_input(&input) {
        Ok(data) => data,
        Err(e) => return e.write_errors().into(),
    };

    let custom_context = if let Some(context) = &validation_data.context {
        if let Some(mutable) = validation_data.mutable {
            if mutable {
                quote!(&'v_a mut #context)
            } else {
                quote!(&'v_a #context)
            }
        } else {
            quote!(&'v_a #context)
        }
    } else {
        quote!(())
    };

    // get all the fields to quote them below
    let mut validation_fields: Vec<ValidateField> = validation_data
        .data
        .take_struct()
        .unwrap()
        .fields
        .into_iter()
        .map(|f| f.parsed)
        // skip fields with #[validate(skip)] attribute
        .filter(|f| if let Some(s) = f.skip { !s } else { true })
        .collect();

    if let Some(nest_all_fields) = validation_data.nest_all_fields {
        if nest_all_fields {
            validation_fields = validation_fields
                .iter_mut()
                .map(|f| {
                    f.nested = Some(true);
                    f.to_owned()
                })
                .collect();
        }
    }

    // generate `use` statements for all used validator traits
    let use_statements = quote_use_stmts(&validation_fields);

    // Schema validation
    let schema = validation_data.schema.iter().fold(quote!(), |acc, s| {
        let st = schema_tokens(s.clone());
        let acc = quote! {
            #acc
            #st
        };
        acc
    });

    let ident = validation_data.ident;
    let (imp, ty, whr) = validation_data.generics.split_for_impl();

    let struct_generics_quote =
        validation_data.generics.params.iter().fold(quote!(), |mut q, g| {
            if let GenericParam::Type(t) = g {
                // Default types are not allowed in trait impl
                if t.default.is_some() {
                    let mut t2 = t.clone();
                    t2.default = None;
                    let g2 = GenericParam::Type(t2);
                    q.extend(quote!(#g2, ));
                } else {
                    q.extend(quote!(#g, ));
                }
            } else {
                q.extend(quote!(#g, ));
            }
            q
        });

    let imp_args = if struct_generics_quote.is_empty() {
        quote!(<'v_a>)
    } else {
        quote!(<'v_a, #struct_generics_quote>)
    };

    let argless_validation = if validation_data.context.is_none() {
        quote! {
            impl #imp ::validator::Validate for #ident #ty #whr {
                fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
                    use validator::ValidateArgs;
                    self.validate_with_args(())
                }
            }
        }
    } else {
        quote!()
    };

    let (validation_fields, constraints): (Vec<_>, Vec<_>) =
        validation_fields.into_iter().map(ValidateField::into_tokens).unzip();

    quote!(
        #argless_validation

        impl #imp_args ::validator::ValidateArgs<'v_a> for #ident #ty #whr {
            type Args = #custom_context;

            fn validate_with_args(&self, args: Self::Args)
            -> ::std::result::Result<(), ::validator::ValidationErrors>
             {
                #use_statements

                let mut errors = ::validator::ValidationErrors::new();

                #(#validation_fields)*

                #schema

                if errors.is_empty() {
                    ::std::result::Result::Ok(())
                } else {
                    ::std::result::Result::Err(errors)
                }
            }
        }

        impl #imp ::validator::Constraints for #ident #ty #whr {
            fn constraints() -> ::validator::ValidationConstraints {
                let mut constraints = ::validator::ValidationConstraints::default();

                #(#constraints)*

                constraints
            }
        }

    )
    .into()
}
