
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput};

//type FieldProps = (String, Type, Visibility);
type FieldProps = (String, String, String);

//struct Field {
//    name: String,
//}

pub fn impl_reflect_macro(ast: DeriveInput) -> TokenStream {
    let ident = ast.ident;
    let ident_str = ident.to_string();

    let field_idents: Vec<FieldProps> = match ast.data {
        Data::Struct(d) => d
            .fields
            .into_iter()
            .filter_map(|f| {
                Some((
                    f.ident.unwrap().to_string(),
                    f.ty.into_token_stream().to_string(),
                    f.vis.into_token_stream().to_string(),
                ))
            })
            .collect(),
        Data::Enum(_) => panic!("Reflect macro only supports Structs."),
        Data::Union(_) => panic!("Reflect macro only supports Structs."),
    };

    let field_idents_str: Vec<String> = field_idents.iter().map(|i| i.0.to_owned()).collect();

    quote! {
        impl Reflect for #ident {
            fn name(&self) -> &'static str {
                #ident_str
            }

            fn field_names(&self) -> Vec<&'static str> {
                vec![#(#field_idents_str), *]
            }
        }
    }
    .into()
}
