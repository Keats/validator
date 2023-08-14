// #![recursion_limit = "128"]

use convert_case::{Case, Casing};
use darling::ast::Data;
use darling::util::Override;
use darling::FromDeriveInput;
use proc_macro_error::proc_macro_error;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Ident, Type};

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

// The field gets converted to tokens in the same format as it was before
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
            custom_tokens(
                match custom {
                    Override::Inherit => Custom::default(),
                    Override::Explicit(c) => c,
                },
                &field_name,
                &field_name_str,
            )
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
// The "supports(struct_named)" should guarantee to only have this
// macro work with structs with named fields I think?
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(validate), supports(struct_named))]
struct ValidationData {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), ValidateField>,
    schema: Option<Override<Schema>>,
}

#[proc_macro_derive(Validate, attributes(validate))]
#[proc_macro_error]
pub fn derive_validation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    // parse the input to the ValidationData struct defined above
    let validation_data = match ValidationData::from_derive_input(&input) {
        Ok(data) => data,
        Err(e) => return e.write_errors().into(),
    };

    // get all the fields to quote them below
    let validation_fields: Vec<ValidateField> = validation_data
        .data
        .take_struct()
        .unwrap()
        .fields
        .into_iter()
        // skip fields with #[validate(skip)] attribute
        .filter(|f| if let Some(s) = f.skip { !s } else { true })
        .collect();

    // generate `use` statements for all used validator traits
    let use_statements = quote_use_stmts(&validation_fields);

    let fields_with_custom_validations: Vec<&ValidateField> =
        validation_fields.iter().filter(|f| f.custom.is_some()).collect();

    // prepare closure idents based on the fields that require them
    let mut custom_validation_closures: Vec<Ident> = fields_with_custom_validations
        .iter()
        .map(|f| format_ident!("{}_closure", f.ident.clone().unwrap()))
        .collect();

    let schema = if let Some(schema) = &validation_data.schema {
        custom_validation_closures.push(format_ident!(
            "{}_schema_closure",
            validation_data.ident.to_string().to_case(Case::Snake)
        ));

        // Schema validation
        schema_tokens(
            match schema {
                Override::Inherit => Schema::default(),
                Override::Explicit(s) => s.clone(),
            },
            &validation_data.ident.clone(),
        )
    } else {
        quote!()
    };

    let generics_for_closures: Vec<Ident> = custom_validation_closures
        .iter()
        .enumerate()
        .map(|(i, _)| format_ident!("A{}", i))
        .collect();

    // generate generics for the impl for ValidateArgs
    let generics_for_impl = if custom_validation_closures.len() > 1 {
        quote!(#(#generics_for_closures, )*)
    } else {
        quote!(#(#generics_for_closures)*)
    };

    // put the generics in a tuple if there's more than one
    let generics_in_parens = if custom_validation_closures.len() > 1 {
        quote!((#(#generics_for_closures, )*))
    } else {
        quote!(#(#generics_for_closures)*)
    };

    let mut types_for_closures: Vec<Type> =
        fields_with_custom_validations.iter().map(|f| f.ty.clone()).collect();

    if validation_data.schema.is_some() {
        types_for_closures
            .push(syn::parse_str::<Type>(&validation_data.ident.to_string()).unwrap());
    }

    let where_clause_for_fn = if custom_validation_closures.len() > 1 {
        quote!(#(#generics_for_closures: FnOnce(&#types_for_closures) -> ::std::result::Result<(), ::validator::ValidationError>, )*)
    } else {
        quote!(#(#generics_for_closures: FnOnce(&#types_for_closures) -> ::std::result::Result<(), ::validator::ValidationError>)*)
    };

    // prepare a destructure parens if there's more than one custom validation
    let destructure_for_args = if custom_validation_closures.len() > 1 {
        quote!((#(#custom_validation_closures, )*))
    } else {
        quote!(#(#custom_validation_closures)*)
    };

    let ident = validation_data.ident;
    let generics = &validation_data.generics.params;
    let (imp, ty, whr) = validation_data.generics.split_for_impl();

    // prepare the impl<...> block with generics
    let arg_imp = if generics.is_empty() {
        quote!(<#generics_for_impl>)
    } else {
        quote!(<#generics, #generics_for_impl>)
    };

    if custom_validation_closures.is_empty() {
        quote! {
            impl #imp ::validator::Validate for #ident #ty #whr {
                fn validate(&self) -> ::std::result::Result<(), ::validator::ValidationErrors> {
                    #use_statements

                    let mut errors = ::validator::ValidationErrors::new();

                    #(#validation_fields)*

                    if errors.is_empty() {
                        ::std::result::Result::Ok(())
                    } else {
                        ::std::result::Result::Err(errors)
                    }
                }
            }
        }
        .into()
    } else {
        quote!(
            impl #arg_imp ::validator::ValidateArgs<#generics_in_parens> for #ident #ty #whr
            where #where_clause_for_fn {
                fn validate(&self, args: #generics_in_parens)
                -> ::std::result::Result<(), ::validator::ValidationErrors>
                 {
                    #use_statements

                    let mut errors = ::validator::ValidationErrors::new();

                    let #destructure_for_args = args;

                    #schema

                    #(#validation_fields)*

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
}
