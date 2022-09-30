use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Parse, attributes(parse))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let ident = &ast.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        panic!("Only structs with named fields are supported");
    }
    .iter();
    let parsed_fields = fields.clone().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote::quote! {
            let (input, #name) = <#ty as Parse>::parse(input)?;
        }
    });

    let short_fields = fields.map(|f| {
        let name = &f.ident;
        quote::quote! {
            #name
        }
    });
    let expanded = quote::quote! {
        impl Parse for #ident {
            fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
                #(#parsed_fields)*
                Ok((
                    input,
                    Self {#(#short_fields,)*}
                ))
            }
        }
    };
    TokenStream::from(expanded)
}
