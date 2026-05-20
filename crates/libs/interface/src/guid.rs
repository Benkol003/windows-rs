//! GUID parsing and token generation for the `#[interface]` macro attribute.

use quote::{ToTokens, quote};

/// The parsed GUID attribute on an `#[interface]` invocation.
///
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
///                              //^ this part is parsed as a Guid
/// Or can pass a GUID value directly as an expression:
/// #[windows_interface::interface(GUID {
//    data1: 0x8ceeb155,
//    data2: 0x2849,
//    data3: 0x4ce5,
//    data4: [0x94, 0x48, 0x91, 0xff, 0x70, 0xe1, 0xe4, 0xd9],
//})]
/// unsafe trait IUIAnimationVariable: IUnknown { ... }
/// ```
///
/// When the attribute is absent (i.e. `#[interface]` with no argument), the GUID is `None`
/// and `to_tokens` emits `GUID::zeroed()`.
pub(crate) enum Guid {
    None,
    LitStr(syn::LitStr),
    Expr(syn::Expr),
}

impl Guid {
    /// Converts the parsed GUID string or expression into a `::windows_core::GUID { ... }` token stream.
    pub(crate) fn to_tokens(&self) -> syn::Result<proc_macro2::TokenStream> {
        fn hex_lit(num: &str) -> syn::LitInt {
            syn::LitInt::new(&format!("0x{num}"), proc_macro2::Span::call_site())
        }

        fn ensure_length(
            part: Option<&str>,
            index: usize,
            length: usize,
            span: proc_macro2::Span,
        ) -> syn::Result<String> {
            let Some(part) = part else {
                return Err(syn::Error::new(
                    span,
                    format!("IID part at index {index} is missing"),
                ));
            };

            if part.len() != length {
                return Err(syn::Error::new(
                    span,
                    format!(
                        "The IID part at index {} must be {} characters long but was {} characters",
                        index,
                        length,
                        part.len()
                    ),
                ));
            }

            Ok(part.to_owned())
        }

        match self {
            Self::None => Ok(quote! {
                ::windows_core::GUID::zeroed()
            }),
            Self::Expr(expr) => Ok(expr.to_token_stream()),
            Self::LitStr(value) => {
                let guid_value = value.value();
                let mut delimited = guid_value.split('-').fuse();
                let chunks = [
                    ensure_length(delimited.next(), 0, 8, value.span())?,
                    ensure_length(delimited.next(), 1, 4, value.span())?,
                    ensure_length(delimited.next(), 2, 4, value.span())?,
                    ensure_length(delimited.next(), 3, 4, value.span())?,
                    ensure_length(delimited.next(), 4, 12, value.span())?,
                ];

                let data1 = hex_lit(&chunks[0]);
                let data2 = hex_lit(&chunks[1]);
                let data3 = hex_lit(&chunks[2]);
                let (data4_1, data4_2) = chunks[3].split_at(2);
                let data4_1 = hex_lit(data4_1);
                let data4_2 = hex_lit(data4_2);
                let (data4_3, rest) = chunks[4].split_at(2);
                let data4_3 = hex_lit(data4_3);
                let (data4_4, rest) = rest.split_at(2);
                let data4_4 = hex_lit(data4_4);
                let (data4_5, rest) = rest.split_at(2);
                let data4_5 = hex_lit(data4_5);
                let (data4_6, rest) = rest.split_at(2);
                let data4_6 = hex_lit(data4_6);
                let (data4_7, data4_8) = rest.split_at(2);
                let data4_7 = hex_lit(data4_7);
                let data4_8 = hex_lit(data4_8);
                Ok(quote! {
                    ::windows_core::GUID {
                        data1: #data1,
                        data2: #data2,
                        data3: #data3,
                        data4: [#data4_1, #data4_2, #data4_3, #data4_4, #data4_5, #data4_6, #data4_7, #data4_8]
                    }
                })
            }
        }
    }
}

impl syn::parse::Parse for Guid {
    fn parse(cursor: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(string) = cursor.parse::<syn::LitStr>() {
            Ok(Self::LitStr(string))
        } else if let Ok(expr) = cursor.parse::<syn::Expr>() {
            Ok(Self::Expr(expr))
        } else {
            Ok(Self::None)
        }
    }
}
