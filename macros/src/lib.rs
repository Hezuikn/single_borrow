use proc_macro::TokenStream;

#[proc_macro]
pub fn impl_reduce(_: TokenStream) -> TokenStream {
    let mut ret = quote::quote! {};
    for i in 0..100 {
        let borrows = std::iter::repeat(quote::quote! {
            &'a
        })
        .take(i)
        .collect::<Vec<_>>();
        ret = quote::quote! {
            #ret
            impl<'a, T: Owned> Reduce<'a> for &'a #(#borrows)* T {
                type OwnedVariant = T;
                #[inline]
                fn reduce(self) -> &'a Self::OwnedVariant {
                    self
                }
            }
        };
    }
    ret.into()
}
