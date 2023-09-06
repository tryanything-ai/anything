#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

enum FieldType {
    Input,
    Output,
    State,
}

struct ParsedFields<'a> {
    recv_fields: Vec<&'a syn::Field>,
    send_fields: Vec<&'a syn::Field>,
}

#[proc_macro_derive(Node, attributes(aggregate, pass_by_ref))]
pub fn node_derive(stream: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(stream as DeriveInput);
    let name = &ast.ident;
    let attributes = &ast.attrs;
    let generics = &ast.generics;
    let data = &ast.data;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut aggregate = false;
    let mut pass_by_ref = false;

    for attr in attributes {
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("aggregate") {
                aggregate = true;
                return Ok(());
            }

            if meta.path.is_ident("pass_by_ref") {
                pass_by_ref = true;
                return Ok(());
            }

            return Err(meta.error("unrecognized attribute"));
        });
    }

    let recv_fields;
    let send_fields;

    match data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => {
                let parsed_fields = parse_fields(fields);
                recv_fields = parsed_fields.recv_fields.clone();
                send_fields = parsed_fields.send_fields.clone();
            }
            _ => {
                let err = quote! {
                    compile_error!("can't derive node for non-named structs");
                };
                return proc_macro::TokenStream::from(err);
            }
        },
        _ => {
            let err = quote! {
                compile_error!("can't derive node for non-named structs");
            };
            return proc_macro::TokenStream::from(err);
        }
    }

    if recv_fields.is_empty() && send_fields.is_empty() {
        let err = quote! {
        compile_error!("node needs at least one NodeReceiver or \
            NodeSender");
        };
        return proc_macro::TokenStream::from(err);
    }

    let recv_idents: Vec<syn::Ident> = recv_fields
        .iter()
        .map(|x| x.ident.clone().unwrap())
        .collect();

    let send_idents: Vec<syn::Ident> = send_fields
        .iter()
        .map(|x| x.ident.clone().unwrap())
        .collect();

    let send_idents1 = &send_idents;
    let send_idents2 = &send_idents;
    let recv_block_idents = &recv_idents;
    let recv_block_fields = &recv_idents;

    let start = quote! {
        fn start(&mut self) {
            #(
                for (send, val) in &self.#send_idents2 {
                    match val {
                        Some(v) => send.send(v.clone()).unwrap(),
                        None => continue,
                    }
                }
            )*
            loop {
                if self.call().is_err() {
                    break;
                }
            }
        }
    };

    let run_func = if pass_by_ref {
        quote! {
            let res = self.run(#(&#recv_block_idents),*)?;
        }
    } else {
        quote! {
            let res = self.run(#(#recv_block_idents),*)?;
        }
    };

    let send_func = if aggregate {
        quote! {
            if let Some(res) = res {
                #(
                    for (send, _) in &self.#send_idents1 {
                        match send.send(res.clone()) {
                            Ok(_) => (),
                            Err(e) => return Err(NodeError::CommError),
                        }
                    }
                )*
            }
        }
    } else {
        quote! {
            #(
                for (send, _) in &self.#send_idents1 {
                    match send.send(res.clone()) {
                        Ok(_) => (),
                        Err(e) => return Err(NodeError::CommError),
                    }
                }
            )*
        }
    };

    let call = quote! {
        fn call(&mut self) -> Result<(), NodeError> {
            #(
                let #recv_block_idents = match self.#recv_block_fields {
                    Some(ref r) => r.recv().or(Err(NodeError::DataEnd))?,
                    None => return Err(NodeError::PermanentError),
                };
            )*
            #run_func
            #send_func
            Ok(())
        }
    };

    let is_connected = quote! {
        fn is_connected(&self) -> bool {
            #(
                if self.#recv_block_fields.is_none() {
                    return false;
                }
            )*
            #(
                if self.#send_idents1.is_empty() {
                    return false;
                }
            )*
            return true;
        }
    };

    let expanded = quote! {
        impl #impl_generics Node for #name #ty_generics #where_clause {
            #start
            #call
            #is_connected
        }
    };

    TokenStream::from(expanded)
}

fn parse_fields(fields: &syn::FieldsNamed) -> ParsedFields {
    let mut recv_fields = vec![];
    let mut send_fields = vec![];
    for field in &fields.named {
        match parse_type(&field) {
            FieldType::Input => recv_fields.push(field),
            FieldType::Output => send_fields.push(field),
            _ => continue,
        }
    }
    ParsedFields {
        recv_fields,
        send_fields,
    }
}

fn parse_type(field: &syn::Field) -> FieldType {
    let ty = &field.ty;
    let type_str = quote! {#ty}.to_string();
    if type_str.starts_with("NodeReceiver") {
        FieldType::Input
    } else if type_str.starts_with("NodeSender") {
        FieldType::Output
    } else {
        FieldType::State
    }
}
