use proc_macro::TokenStream;

use syn::{
    Ident, LitInt, LitStr, Token,
    parse::{Parse, ParseStream, Result},
};

#[derive(Debug)]
struct TaskParams {
    queue_name: String,
    task_name: String,
    max_retries: u32,
}

impl Parse for TaskParams {
    fn parse(input: ParseStream) -> Result<Self> {
        let queue_name_ident: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let queue_name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        let task_name_ident: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let task_name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        let max_retries_ident: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let max_retries: LitInt = input.parse()?;

        Ok(TaskParams {
            queue_name: queue_name.value(),
            task_name: task_name.value(),
            max_retries: max_retries.base10_parse()?,
        })
    }
}

pub fn task_meta_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let params = syn::parse_macro_input!(args as TaskParams);
    let input_fn = syn::parse_macro_input!(input as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let vis = &input_fn.vis;
    let attrs = &input_fn.attrs;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    // 提取输入输出类型
    let inputs = &sig.inputs;
    let output = &sig.output;
    let input_ty = if inputs.len() == 1 {
        if let syn::FnArg::Typed(pat_ty) = inputs.first().unwrap() {
            &pat_ty.ty
        } else {
            panic!("task fn must have one argument")
        }
    } else {
        panic!("task fn must have one argument")
    };

    let input_pat = if let syn::FnArg::Typed(pat_ty) = inputs.first().unwrap() {
        &pat_ty.pat
    } else {
        panic!("task fn must have one argument")
    };

    let output_ty = match output {
        syn::ReturnType::Default => quote::quote! { () },
        syn::ReturnType::Type(_, ty) => {
            let mut found = None;
            if let syn::Type::Path(type_path) = &**ty {
                let segments = &type_path.path.segments;
                if let Some(seg) = segments.last() {
                    if seg.ident == "Result" {
                        if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                            if let Some(syn::GenericArgument::Type(ok_ty)) = args.args.first() {
                                // Ok 是 Option<T>
                                if let syn::Type::Path(opt_path) = ok_ty {
                                    if let Some(opt_seg) = opt_path.path.segments.last() {
                                        if opt_seg.ident == "Option" {
                                            if let syn::PathArguments::AngleBracketed(opt_args) =
                                                &opt_seg.arguments
                                            {
                                                if let Some(syn::GenericArgument::Type(inner_ty)) =
                                                    opt_args.args.first()
                                                {
                                                    found = Some(quote::quote! { #inner_ty });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            found.unwrap_or_else(|| quote::quote! { #ty })
        }
    };

    // 生成结构体名
    // 结构体名直接用函数名
    // 将函数名（可能为蛇形）转为驼峰并加 Task 后缀
    let fn_name_str = fn_name.to_string();
    let mut struct_name_str = String::new();
    let mut upper = true;
    for c in fn_name_str.chars() {
        if c == '_' {
            upper = true;
        } else if upper {
            struct_name_str.push(c.to_ascii_uppercase());
            upper = false;
        } else {
            struct_name_str.push(c);
        }
    }
    struct_name_str.push_str("Task");
    let struct_name = syn::Ident::new(&struct_name_str, fn_name.span());
    let queue_name = params.queue_name;
    let task_name = params.task_name;
    let max_retries = params.max_retries;
    let expanded = quote::quote! {
         #(#attrs)*
        #vis #sig #block

        #[allow(non_camel_case_types, missing_docs)]
        pub struct #struct_name;
        impl TaskMeta for #struct_name {
            type Input = #input_ty;
            type Output = #output_ty;
            fn queue_name() -> &'static str { #queue_name }
            fn task_name() -> &'static str { #task_name }
            fn handler(#input_pat: Self::Input) -> Result<Option<Self::Output>, CeleryError> {
                #block
            }
            fn max_retries() -> u32 { #max_retries }
        }
    };
    TokenStream::from(expanded)
}
