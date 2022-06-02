use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

const SINGLETON_DEFAULT: bool = false;

#[proc_macro_attribute]
pub fn provider(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let input = parse_macro_input!(input as syn::ItemFn);
    provider_impl(&args, &input)
}

fn provider_impl(args: &syn::AttributeArgs, input: &syn::ItemFn) -> TokenStream {
    let input_fn_name = input.sig.ident.clone();
    let config_type_name = get_config_type_name(input);
    let (output_type_name, output_type_params) = get_output_type_name(input);
    let is_singleton = get_singleton_tag(args);

    let log_creating = format!(
        "Type '{}' is not global, creating new instance.",
        output_type_name.to_string()
    );
    let output_type_name_as_str = output_type_name.to_string();

    let fn_call = match is_singleton {
        false => quote! {
            debug!(#log_creating);
            #input_fn_name(self)
        },
        true => quote! {
            self.resolve_singleton(|context| {
                #input_fn_name(context)
            }, #output_type_name_as_str)

        },
    };

    let gen = quote! {
        impl crate::dose_private::Injector<#output_type_name #output_type_params> for dose::Context<#config_type_name> {
            fn get(&mut self) -> #output_type_name #output_type_params {
                #fn_call
            }
        }
        #input
    };
    gen.into()
}

fn get_singleton_tag(args: &syn::AttributeArgs) -> bool {
    let singleton_tag = match args.first() {
        Some(val) => val,
        None => return SINGLETON_DEFAULT,
    };
    let singleton_tag = match singleton_tag {
        syn::NestedMeta::Meta(meta) => meta,
        _ => panic!("Resolver trait not provided"),
    };
    let tag_name = singleton_tag.path().segments.first().unwrap().ident.clone();
    if tag_name.to_string() != "singleton" {
        return false;
    }
    let tag_value = match singleton_tag {
        syn::Meta::NameValue(val) => val,
        _ => return false,
    };
    let tag_value = match &tag_value.lit {
        syn::Lit::Bool(val) => val,
        _ => panic!("Should be a bool"),
    };

    tag_value.value
}

fn get_output_type_name(input: &syn::ItemFn) -> (syn::Ident, syn::PathArguments) {
    let output_type = match &input.sig.output {
        syn::ReturnType::Type(_, b) => b.as_ref().clone(),
        _ => panic!("No output to the function"),
    };
    let output_type = match output_type {
        syn::Type::Path(path) => path,
        _ => panic!("Unsupported type"),
    };
    let output_type = output_type.path.segments.first().unwrap();

    (output_type.ident.clone(), output_type.arguments.clone())
}

fn get_config_type_name(input: &syn::ItemFn) -> syn::Ident {
    let config_type = match input.sig.inputs.first() {
        Some(ty) => ty,
        None => panic!("Function need a first argument"),
    };
    let config_type = match config_type {
        syn::FnArg::Typed(ty) => ty,
        _ => panic!("Not typed"),
    };
    let config_type = match &*config_type.ty {
        syn::Type::Reference(r) => r,
        _ => panic!("Argument should be a reference to the resolver"),
    };
    let config_type = match &*config_type.elem {
        syn::Type::Path(path) => path,
        _ => panic!("Argument should be a reference to the resolver"),
    };
    let config_type = config_type.path.segments.first().unwrap();
    let config_type = match &config_type.arguments {
        syn::PathArguments::AngleBracketed(br) => br,
        _ => panic!("Argument should be a reference to the resolver"),
    };
    let config_type = config_type.args.first().unwrap();
    let config_type = match config_type {
        syn::GenericArgument::Type(ty) => ty,
        _ => panic!("Argument should be a reference to the resolver"),
    };
    let config_type = match &*config_type {
        syn::Type::Path(path) => path,
        _ => panic!("Argument should be a reference to the resolver"),
    };
    let config_type = config_type.path.segments.first().unwrap();
    let config_type = config_type.ident.clone();

    config_type
}
