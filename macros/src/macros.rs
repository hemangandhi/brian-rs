use syn::braced;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Error, Expr, Ident, Token, Type};

use proc_macro2::{Punct, Spacing, TokenStream};

use quote::ToTokens;
use quote::TokenStreamExt;

// Thanks Obama
fn syn_ident_to_proc2(syn_id: &Ident) -> proc_macro2::Ident {
    proc_macro2::Ident::new(&syn_id.to_string(), syn_id.span())
}

fn expect_str(input: &ParseStream, s: &str) -> Result<()> {
    let ident: Ident = input.parse()?;
    if ident.to_string() != s {
        return Err(Error::new(
            ident.span(),
            format!("Expected `{}` keyword", s),
        ));
    }
    Ok(())
}

fn get_delimited_within_braces<T: Parse, Delim: syn::token::Token + Parse>(
    input: &ParseStream,
) -> Result<Vec<T>> {
    let inits_toks;
    braced!(inits_toks in input);
    Ok(
        Punctuated::<T, Delim>::parse_separated_nonempty(&inits_toks)?
            .into_iter()
            .collect(),
    )
}

pub struct NeuronType {
    pub type_name: Ident,
    pub voltage_type: Type,
    pub time_type: Type,
}

impl Parse for NeuronType {
    fn parse(input: ParseStream) -> Result<Self> {
        let type_name: Ident = input.parse()?;
        input.parse::<Token![<]>()?;
        let voltage_type: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let time_type: Type = input.parse()?;
        input.parse::<Token![>]>()?;
        Ok(NeuronType {
            type_name,
            voltage_type,
            time_type,
        })
    }
}

#[derive(Clone)]
pub struct IdentAndType {
    pub name: Ident,
    type_name: Type,
}

impl Parse for IdentAndType {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let type_name: Type = input.parse()?;
        Ok(IdentAndType {
            name: name,
            type_name: type_name,
        })
    }
}

impl ToTokens for IdentAndType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        syn_ident_to_proc2(&self.name).to_tokens(tokens);
        tokens.append(Punct::new(':', Spacing::Alone));
        self.type_name.to_tokens(tokens);
    }
}

#[derive(Clone)]
pub struct IdentAndValue {
    name: Ident,
    typ: Type,
    value: Expr,
}

impl IdentAndValue {
    pub fn drop_value(&self) -> IdentAndType {
        IdentAndType {
            name: self.name.clone(),
            type_name: self.typ.clone(),
        }
    }
}

impl Parse for IdentAndValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let typ: Type = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: Expr = input.parse()?;
        Ok(IdentAndValue {
            name: name,
            typ: typ,
            value: value,
        })
    }
}

/// Note: there are two tokenizations here:
/// 1) drop_value(self).to_tokens(tokens) gives something apt for a
///    forward declaration (ident: type, ie. in a fn param list)
/// 2) self.to_tokens(tokens) gives something apt for assignment
///    (ident: value, ie. in a constructor for a struct).
impl ToTokens for IdentAndValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        tokens.append(Punct::new(':', Spacing::Alone));
        self.value.to_tokens(tokens);
    }
}

pub enum IdentOrDerivative {
    Ident(Ident),
    Derivative(Ident),
}

impl Parse for IdentOrDerivative {
    fn parse(input: ParseStream) -> Result<Self> {
        let id: Ident = input.parse()?;
        let next = input.lookahead1();
        if next.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            Ok(IdentOrDerivative::Derivative(id))
        } else {
            Ok(IdentOrDerivative::Ident(id))
        }
    }
}

pub struct Equation {
    left_side: IdentOrDerivative,
    right_side: Expr,
}

impl Parse for Equation {
    fn parse(input: ParseStream) -> Result<Self> {
        let lhs: IdentOrDerivative = input.parse()?;
        input.parse::<Token![=]>()?;
        let rhs: Expr = input.parse()?;
        Ok(Equation {
            left_side: lhs,
            right_side: rhs,
        })
    }
}

impl ToTokens for Equation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lhs = match &self.left_side {
            IdentOrDerivative::Derivative(i) => {
                i
            }
            IdentOrDerivative::Ident(i) => i,
        };
        tokens.append(proc_macro2::Ident::new("self", lhs.span()));
        tokens.append(Punct::new('.', Spacing::Joint));
        lhs.to_tokens(tokens);
        tokens.append(Punct::new('=', Spacing::Alone));

	let mut eqn_toks = TokenStream::new();
	self.right_side.to_tokens(&mut eqn_toks);
	tokens.append(proc_macro2::Group::new(
            proc_macro2::Delimiter::Parenthesis,
            eqn_toks,
        ));

        if let IdentOrDerivative::Derivative(_) = self.left_side {
            tokens.append(Punct::new('*', Spacing::Alone));
            tokens.append(proc_macro2::Ident::new("dt", lhs.span()));
            tokens.append(Punct::new('+', Spacing::Alone));
            tokens.append(proc_macro2::Ident::new("self", lhs.span()));
            tokens.append(Punct::new('.', Spacing::Joint));
            lhs.to_tokens(tokens);
        }
        tokens.append(Punct::new(';', Spacing::Alone));
    }
}

pub struct NeuronDef {
    pub neuron_type: NeuronType,
    pub param_list: Vec<IdentAndType>,
    pub initialize_list: Vec<IdentAndValue>,
    pub time_step: Vec<Equation>,
    pub spike_when: Expr,
    pub voltage_getter: Expr,
    pub reset: Vec<Expr>,
}

impl Parse for NeuronDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let typ: NeuronType = input.parse()?;
        input.parse::<Token![:]>()?;
        expect_str(&input, "params")?;
        let params = get_delimited_within_braces::<IdentAndType, Token![,]>(&input)?;
        expect_str(&input, "initialize")?;
        let inits = get_delimited_within_braces::<IdentAndValue, Token![;]>(&input)?;
        expect_str(&input, "time_step")?;
        let time_steps = get_delimited_within_braces::<Equation, Token![;]>(&input)?;
        expect_str(&input, "spike_when")?;
        let spike_cond: Expr = {
            let spike_cond_toks;
            braced!(spike_cond_toks in input);
            spike_cond_toks.parse()?
        };
        expect_str(&input, "get_voltage")?;
        let voltage_getter: Expr = {
            let voltage_toks;
            braced!(voltage_toks in input);
            voltage_toks.parse()?
        };
        expect_str(&input, "reset")?;
        let resets = get_delimited_within_braces::<Expr, Token![;]>(&input)?;
        Ok(NeuronDef {
            neuron_type: typ,
            param_list: params,
            initialize_list: inits,
            time_step: time_steps,
            spike_when: spike_cond,
            voltage_getter: voltage_getter,
            reset: resets,
        })
    }
}

pub struct SynapseDef {
    pub synapse_type: NeuronType,
    pub param_list: Vec<IdentAndType>,
    pub initialize_list: Vec<IdentAndValue>,
    pub time_step: Vec<Equation>,
    pub weight_getter: Expr,
    pub pre_synapse_spike: Vec<Expr>,
    pub post_synapse_spike: Vec<Expr>,
}

impl Parse for SynapseDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let typ: NeuronType = input.parse()?;
        input.parse::<Token![:]>()?;
        expect_str(&input, "params")?;
        let params = get_delimited_within_braces::<IdentAndType, Token![,]>(&input)?;
        expect_str(&input, "initialize")?;
        let inits = get_delimited_within_braces::<IdentAndValue, Token![;]>(&input)?;
        expect_str(&input, "time_step")?;
        let time_steps = get_delimited_within_braces::<Equation, Token![;]>(&input)?;
        expect_str(&input, "weight_getter")?;
        let weight_getter: Expr = {
            let weight_toks;
            braced!(weight_toks in input);
            weight_toks.parse()?
        };
        expect_str(&input, "on_pre")?;
        let pre_spike = get_delimited_within_braces::<Expr, Token![;]>(&input)?;
        expect_str(&input, "on_post")?;
        let post_spike = get_delimited_within_braces::<Expr, Token![;]>(&input)?;
        Ok(SynapseDef {
            synapse_type: typ,
            param_list: params,
            initialize_list: inits,
            time_step: time_steps,
            weight_getter: weight_getter,
            pre_synapse_spike: pre_spike,
            post_synapse_spike: post_spike,
        })
    }
}
