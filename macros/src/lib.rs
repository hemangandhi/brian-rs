extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Ident;

mod macros;

use crate::macros::{IdentAndType, NeuronDef, NeuronType};

#[proc_macro]
pub fn define_neuron(input: TokenStream) -> TokenStream {
    let NeuronDef {
        neuron_type,
        param_list,
        initialize_list,
        time_step,
        spike_when,
        current_getter,
        reset,
    } = parse_macro_input!(input as NeuronDef);
    let field_names: Vec<Ident> = param_list.iter().map(|i| i.name.clone()).collect();
    let typed_fields: Vec<IdentAndType> = param_list
        .clone()
        .into_iter()
        .chain(initialize_list.clone().into_iter().map(|i| i.drop_value()))
        .collect();
    // Note: current means electric current.
    let NeuronType {
        type_name,
        current_type,
        time_type,
    } = neuron_type;
    let expanded = quote! {
    pub struct #type_name {
        #(#typed_fields),*
    }

    impl #type_name {
        pub fn new(#(#param_list),*) -> Self {
            #type_name {
        #(#field_names),*,
        #(#initialize_list),*
            }
        }
    }

    impl SpikeGenerator<#current_type> for #type_name {
        fn did_spike(&self) -> bool { #spike_when }
        fn get_current(&self) -> #current_type { #current_getter }
    }

    impl InnerSpikeGenerator<#current_type, #time_type> for #type_name {
        fn handle_input(&mut self, input: #current_type, dt: #time_type) {
        if self.did_spike() { #(#reset);*; }
        #(#time_step);*;
    }
    }
    };
    expanded.into()
}
