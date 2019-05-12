extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use syn::(Type, Ident, Expr);
use syn::parse::(Parse, Token, ParseStream, Result);

pub mod macros{

    struct IdentAndType{
        name: Ident,
        type_name: Type
    }

    impl Parse for IdentAndType{
        fn parse(input: ParseStream) -> Result<self> {
            let name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let type_name: Type = input.parse()?;
            Ok(IdentAndType {
                name: name,
                type_name: type_name
            })
        }
    }

    struct IdentAndValue{
        name: Ident,
        value: Expr
    }

    impl Parse for IdentAndValue{
        fn parse(input: ParseStream) -> Result<self> {
            let name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let type_name: Expr = input.parse()?;
            Ok(IdentAndValue {
                name: name,
                value: type_name
            })
        }
    }

    enum IdentOrDerivative {
        Ident(Ident),
        Derivative(Ident)
    }

    impl Parse for IdentOrDerivative{
        fn parse(input: ParseStream) -> Result<self> {
            
        }
    }

    struct Equation {
        left_side: IdentOrDerivative,
        right_side: Expr
    }

    struct NeuronDef {
        type_name: Type,
        param_list: Vec<IdentAndType>,
        initialize_list: Vec<IdentAndValue>,
        time_step: Vec<Equation>,
        spike_when: Expr,
        reset: Vec<Equation>
    }

    #[proc_macro]
    pub fn define_neuron(input: TokenStream) -> TokenStream {
        
    }

}
