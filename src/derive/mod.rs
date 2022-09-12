use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta, MetaNameValue, spanned::Spanned};
use indoc::indoc;

macro_rules! bail {( $err_msg:expr$(, $span:expr)? $(,)? ) => (
    {
        let mut _span = ::proc_macro2::Span::call_site();
        $( _span = $span; )?
        return ::syn::Error::new(_span, $err_msg)
                   .to_compile_error()
                   .into()
        ;
    }
)}

#[proc_macro_derive(Label, attributes(label))]
pub fn label(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let DeriveInput {
        ident,
        attrs,
        data: _,
        vis: _,
        generics: _,
    } = parse_macro_input!(input);

    // find the attribute we care about
    let label_attr = attrs
        .iter()
        .find(|attr| attr.path.is_ident("label"));

    // bail if it doesn't exist
    if label_attr.is_none() {
        bail!(
            r#"missing label attribute, e.g. #[label = "My label"]"#,
            ident.span()
        )
    }

    match label_attr.unwrap().parse_meta() {
        Ok(Meta::NameValue(MetaNameValue {
            lit: Lit::Str(label),
            eq_token: _,
            path: _
        })) => {
            // Build the output, possibly using quasi-quotation
            let expanded = quote! {
                impl ::type_label::Label for #ident {
                    const LABEL: &'static str = #label;
                }
            };

            // Hand the output tokens back to the compiler
            TokenStream::from(expanded)
        },
        Ok(Meta::NameValue(MetaNameValue {
            lit: bad,
            eq_token: _,
            path: _
        })) => bail!(
            indoc! {r#"
                expected a string label, e.g. #[label = "My label"]
                                                        ^^^^^^^^^^ i.e. this part needs to be a string, with quotes!
            "#},            
            bad.span()
        ),
        Ok(bad) => bail!(indoc!{r#"
                expected name value syntax e.g. #[label = "My label"]
                                                       ^^^ i.e. this eq sign is needed, not #[Label("My label")]
            "#},            
            bad.span()
        ),
        Err(_) => bail!(indoc!{r#"
            Error parsing label attribute.
            
            Your label helper attribute should be above the type deriving Label,
            just below the #[derive(Label)].
            e.g.
                #[derive(Label, Clone, Debug, ...)]
                #[label = "My label"]
                struct MyStruct {
        "#}),
    }
}
