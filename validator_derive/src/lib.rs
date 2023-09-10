use darling::ast::Data;
use darling::util::{Override, WithOriginal};
use darling::FromDeriveInput;
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Field, Path, PathArguments};

use tokens::cards::credit_card_tokens;
use tokens::contains::contains_tokens;
use tokens::custom::custom_tokens;
use tokens::does_not_contain::does_not_contain_tokens;
use tokens::email::email_tokens;
use tokens::ip::ip_tokens;
use tokens::length::length_tokens;
use tokens::must_match::must_match_tokens;
use tokens::nested::nested_tokens;
use tokens::non_control_character::non_control_char_tokens;
use tokens::range::range_tokens;
use tokens::regex::regex_tokens;
use tokens::required::required_tokens;
use tokens::required_nested::required_nested_tokens;
use tokens::schema::schema_tokens;
use tokens::url::url_tokens;
use types::*;
use utils::quote_use_stmts;

mod tokens;
mod types;
mod utils;

impl ToTokens for ValidateField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let field_name = self.ident.clone().unwrap();
        let field_name_str = self.ident.clone().unwrap().to_string();

        // Length validation
        let length = if let Some(length) = self.length.clone() {
            length_tokens(length, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Email validation
        let email = if let Some(email) = self.email.clone() {
            email_tokens(
                match email {
                    Override::Inherit => Email::default(),
                    Override::Explicit(e) => e,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Credit card validation
        let card = if let Some(credit_card) = self.credit_card.clone() {
            credit_card_tokens(
                match credit_card {
                    Override::Inherit => Card::default(),
                    Override::Explicit(c) => c,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Url validation
        let url = if let Some(url) = self.url.clone() {
            url_tokens(
                match url {
                    Override::Inherit => Url::default(),
                    Override::Explicit(u) => u,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Ip address validation
        let ip = if let Some(ip) = self.ip.clone() {
            ip_tokens(
                match ip {
                    Override::Inherit => Ip::default(),
                    Override::Explicit(i) => i,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Non control character validation
        let ncc = if let Some(ncc) = self.non_control_character.clone() {
            non_control_char_tokens(
                match ncc {
                    Override::Inherit => NonControlCharacter::default(),
                    Override::Explicit(n) => n,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Range validation
        let range = if let Some(range) = self.range.clone() {
            range_tokens(range, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Required validation
        let required = if let Some(required) = self.required.clone() {
            required_tokens(
                match required {
                    Override::Inherit => Required::default(),
                    Override::Explicit(r) => r,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Required nested validation
        let required_nested = if let Some(required_nested) = self.required_nested.clone() {
            required_nested_tokens(
                match required_nested {
                    Override::Inherit => Required::default(),
                    Override::Explicit(r) => r,
                },
                &field_name,
                &field_name_str,
            )
        } else {
            quote!()
        };

        // Contains validation
        let contains = if let Some(contains) = self.contains.clone() {
            contains_tokens(contains, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Does not contain validation
        let does_not_contain = if let Some(does_not_contain) = self.does_not_contain.clone() {
            does_not_contain_tokens(does_not_contain, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Must match validation
        let must_match = if let Some(must_match) = self.must_match.clone() {
            must_match_tokens(must_match, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Regex validation
        let regex = if let Some(regex) = self.regex.clone() {
            regex_tokens(regex, &field_name, &field_name_str)
        } else {
            quote!()
        };

        // Custom validation
        let custom = if let Some(custom) = self.custom.clone() {
            custom_tokens(custom, &field_name, &field_name_str)
        } else {
            quote!()
        };

        let nested = if let Some(n) = self.nested {
            if n {
                nested_tokens(&field_name, &field_name_str)
            } else {
                quote!()
            }
        } else {
            quote!()
        };

        tokens.extend(quote! {
            #length
            #email
            #card
            #url
            #ip
            #ncc
            #range
            #required
            #required_nested
            #contains
            #does_not_contain
            #must_match
            #regex
            #custom
            #nested
        });
    }
}

// The main struct we get from parsing the attributes
// The "supports(struct_named)" attribute guarantees only named structs to work with this macro
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(validate), supports(struct_named))]
#[darling(and_then = ValidationData::validate)]
struct ValidationData {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), WithOriginal<ValidateField, syn::Field>>,
    schema: Option<Schema>,
    context: Option<Path>,
    mutable: Option<bool>,
    nested: Option<bool>,
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
    let schema = if let Some(schema) = &validation_data.schema {
        schema_tokens(schema.clone())
    } else {
        quote!()
    };

    let ident = validation_data.ident;
    let (imp, ty, whr) = validation_data.generics.split_for_impl();

    let struct_generics_quote =
        validation_data.generics.params.iter().fold(quote!(), |mut q, g| {
            q.extend(quote!(#g, ));
            q
        });

    let imp_args = if struct_generics_quote.is_empty() {
        quote!(<'v_a>)
    } else {
        quote!(<'v_a, #struct_generics_quote>)
    };

    let nested_validation = if validation_data.nested.is_some_and(|n| n) {
        quote! {
            impl #imp_args ::validator::ValidateNested<'v_a> for #ident #ty #whr {
                type Args = #custom_context;
                fn validate_nested(&self, field_name: &'static str, args: Self::Args) -> ::std::result::Result<(), ::validator::ValidationErrors> {
                    use validator::ValidateArgs;
                    let res = self.validate_with_args(args);

                    if let Err(e) = res {
                        let new_err = validator::ValidationErrorsKind::Struct(::std::boxed::Box::new(e));
                        std::result::Result::Err(validator::ValidationErrors(::std::collections::HashMap::from([(field_name, new_err)])))
                    } else {
                        std::result::Result::Ok(())
                    }
                }
            }
        }
    } else {
        quote!()
    };

    let argless_validation = if validation_data.context.is_none() {
        quote! {
            impl #imp ::validator::Validate for #ident #ty #whr {
                fn validate(&self) -> Result<(), ::validator::ValidationErrors> {
                    use validator::ValidateArgs;
                    self.validate_with_args(())
                }
            }
        }
    } else {
        quote!()
    };

    quote!(
        #argless_validation

        #nested_validation

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
    )
    .into()
}
