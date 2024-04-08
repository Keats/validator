use darling::ast::Data;
use darling::util::{Override, WithOriginal};
use darling::FromDeriveInput;
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Field, GenericParam, Path, PathArguments};

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

        let type_name = self.ty.to_token_stream().to_string();
        let is_number = NUMBER_TYPES.contains(&type_name);

        let (actual_field, wrapper_closure) = self.if_let_option_wrapper(&field_name, is_number);

        // Length validation
        let length = if let Some(length) = self.length.clone() {
            wrapper_closure(length_tokens(length, &actual_field, &field_name_str))
        } else {
            quote!()
        };

        // Email validation
        let email = if let Some(email) = self.email.clone() {
            wrapper_closure(email_tokens(
                match email {
                    Override::Inherit => Email::default(),
                    Override::Explicit(e) => e,
                },
                &actual_field,
                &field_name_str,
            ))
        } else {
            quote!()
        };

        // Credit card validation
        let card = if let Some(credit_card) = self.credit_card.clone() {
            wrapper_closure(credit_card_tokens(
                match credit_card {
                    Override::Inherit => Card::default(),
                    Override::Explicit(c) => c,
                },
                &actual_field,
                &field_name_str,
            ))
        } else {
            quote!()
        };

        // Url validation
        let url = if let Some(url) = self.url.clone() {
            wrapper_closure(url_tokens(
                match url {
                    Override::Inherit => Url::default(),
                    Override::Explicit(u) => u,
                },
                &actual_field,
                &field_name_str,
            ))
        } else {
            quote!()
        };

        // Ip address validation
        let ip = if let Some(ip) = self.ip.clone() {
            wrapper_closure(ip_tokens(
                match ip {
                    Override::Inherit => Ip::default(),
                    Override::Explicit(i) => i,
                },
                &actual_field,
                &field_name_str,
            ))
        } else {
            quote!()
        };

        // Non control character validation
        let ncc = if let Some(ncc) = self.non_control_character.clone() {
            wrapper_closure(non_control_char_tokens(
                match ncc {
                    Override::Inherit => NonControlCharacter::default(),
                    Override::Explicit(n) => n,
                },
                &actual_field,
                &field_name_str,
            ))
        } else {
            quote!()
        };

        // Range validation
        let range = if let Some(range) = self.range.clone() {
            wrapper_closure(range_tokens(range, &actual_field, &field_name_str))
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

        // Contains validation
        let contains = if let Some(contains) = self.contains.clone() {
            wrapper_closure(contains_tokens(contains, &actual_field, &field_name_str))
        } else {
            quote!()
        };

        // Does not contain validation
        let does_not_contain = if let Some(does_not_contain) = self.does_not_contain.clone() {
            wrapper_closure(does_not_contain_tokens(
                does_not_contain,
                &actual_field,
                &field_name_str,
            ))
        } else {
            quote!()
        };

        // Must match validation
        let must_match = if let Some(must_match) = self.must_match.clone() {
            // TODO: handle option for other
            wrapper_closure(must_match_tokens(must_match, &actual_field, &field_name_str))
        } else {
            quote!()
        };

        // Regex validation
        let regex = if let Some(regex) = self.regex.clone() {
            wrapper_closure(regex_tokens(regex, &actual_field, &field_name_str))
        } else {
            quote!()
        };

        // Custom validation
        let mut custom = quote!();
        // We try to be smart when passing arguments
        let is_cow = type_name.contains("Cow <");
        let custom_actual_field = if is_cow {
            quote!(#actual_field.as_ref())
        } else if is_number || type_name.starts_with("&") {
            quote!(#actual_field)
        } else {
            quote!(&#actual_field)
        };

        for c in &self.custom {
            let tokens = custom_tokens(c.clone(), &custom_actual_field, &field_name_str);
            custom = quote!(
                #custom

                #tokens
            );
        }
        if !self.custom.is_empty() {
            custom = wrapper_closure(custom);
        }

        let nested = if let Some(n) = self.nested {
            if n {
                wrapper_closure(nested_tokens(&actual_field, &field_name_str))
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
    )
    .into()
}
