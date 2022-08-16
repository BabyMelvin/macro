use proc_macro::TokenStream;
use quote::quote;
use syn;

/// syn 将字符串形式的 Rust 代码解析为一个 AST 树的数据结构
/// ，该数据结构可以在随后的 impl_hello_macro 函数中进行操作。
///
/// 操作的结果又会被 quote 包转换回 Rust 代码。
///
/// syn::parse 调用会返回一个 DeriveInput 结构体来代表解析后的 Rust 代码:
/// DeriveInput {
//     // --snip--
//     ident: Ident {
//         ident: "Sunfei",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }

// 以上就是源代码 struct Sunfei; 解析后的结果，里面有几点值得注意:
//  fields: Unit 说明源代码是一个单元结构体
//  ident: "Sunfei" 说明类型名称为 Sunfei， ident 是标识符 identifier 的简写

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast = syn::parse(input).unwrap();

    // 构建特征代码实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}