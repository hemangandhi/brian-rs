extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result, Token};
use syn::{Expr, Ident, Type};

pub mod macros {

    struct IdentAndType {
        name: Ident,
        type_name: Type,
    }

    impl Parse for IdentAndType {
        fn parse(input: ParseStream) -> Result<self> {
            let name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let type_name: Type = input.parse()?;
            Ok(IdentAndType {
                name: name,
                type_name: type_name,
            })
        }
    }

    struct IdentAndValue {
        name: Ident,
        value: Expr,
    }

    impl Parse for IdentAndValue {
        fn parse(input: ParseStream) -> Result<self> {
            let name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;
            Ok(IdentAndValue {
                name: name,
                value: value,
            })
        }
    }

    enum IdentOrDerivative {
        Ident(Ident),
        Derivative(Ident),
    }

    impl Parse for IdentOrDerivative {
        fn parse(input: ParseStream) -> Result<self> {
            let id: Ident = input.parse();
            let next = input.lookahead1();
            if next.peek(Token![@]) {
                IdentOrDerivative::Derivative(id)
            } else {
                IdentOrDerivative::Ident(id)
            }
        }
    }

    struct Equation {
        left_side: IdentOrDerivative,
        right_side: Expr,
    }

    impl Parse for Equation {
        fn parse(input: ParseStream) -> Result<self> {
            let lhs: IdentOrDerivative = input.parse();
            input.parse::<Token![=]>();
            let rhs: Expr = input.parse();
            Ok(Equation {
                left_side: lhs,
                right_side: rhs,
            })
        }
    }

    struct NeuronDef {
        type_name: Type,
        param_list: Vec<IdentAndType>,
        initialize_list: Vec<IdentAndValue>,
        time_step: Vec<Equation>,
        spike_when: Expr,
        reset: Vec<Equation>,
    }

    impl Parse for NeuronDef {
        fn parse(input: ParseStream) -> Result<self> {
            let typ: Type = input.parse();
            input.parse::<Token![,]>();
        }
    }

    #[proc_macro]
    pub fn define_neuron(input: TokenStream) -> TokenStream {}

}
