extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Ident;

mod macros;

use crate::macros::{IdentAndType, NeuronDef, NeuronType, SynapseDef};

#[proc_macro]
pub fn define_neuron(input: TokenStream) -> TokenStream {
    let NeuronDef {
        neuron_type,
        param_list,
        initialize_list,
        time_step,
        spike_when,
        voltage_getter,
        reset,
    } = parse_macro_input!(input as NeuronDef);
    let field_names: Vec<Ident> = param_list.iter().map(|i| i.name.clone()).collect();
    let typed_fields: Vec<IdentAndType> = param_list
        .clone()
        .into_iter()
        .chain(initialize_list.clone().into_iter().map(|i| i.drop_value()))
        .collect();
    // Note: voltage means electric voltage.
    let NeuronType {
        type_name,
        voltage_type,
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

    impl SpikeGenerator<#voltage_type> for #type_name {
        fn did_spike(&self) -> bool { #spike_when }
        fn get_voltage(&self) -> #voltage_type { #voltage_getter }
    }

    impl InnerSpikeGenerator<#voltage_type, #time_type> for #type_name {
        fn handle_input(&mut self, input: #voltage_type, dt: #time_type) {
        if self.did_spike() { #(#reset);*; }
        #(#time_step);*;
    }
    }
    };
    expanded.into()
}

#[proc_macro]
pub fn define_synapse(input: TokenStream) -> TokenStream {
    let SynapseDef {
        synapse_type,
        param_list,
        initialize_list,
	time_step,
        weight_getter,
        pre_synapse_spike,
        post_synapse_spike,
    } = parse_macro_input!(input as SynapseDef);
    let field_names: Vec<Ident> = param_list.iter().map(|i| i.name.clone()).collect();
    let typed_fields: Vec<IdentAndType> = param_list
        .clone()
        .into_iter()
        .chain(initialize_list.clone().into_iter().map(|i| i.drop_value()))
        .collect();
    // Note: voltage means electric voltage.
    let NeuronType {
        type_name,
        voltage_type,
        time_type,
    } = synapse_type;
    let expanded = quote!{
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

    impl Synaptic<#voltage_type, #time_type> for #type_name {
        fn on_pre(&mut self, input: #voltage_type) { #(#pre_synapse_spike);*; }	    
        fn on_post(&mut self, input: #voltage_type) { #(#post_synapse_spike);*; }
	fn current_weight(&mut self) -> f64 { #weight_getter }

	fn advance_once(&mut self, dt: #time_type) {
            #(#time_step);*;
	}
    }
    };
    expanded.into()
}
