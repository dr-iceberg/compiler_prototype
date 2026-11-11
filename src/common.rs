use crate::analyzer::FunctionSignature;
use crate::ast::NodeId;
use crate::grammar::{Parameter, Type};
use crate::util::{self, BinVec};

pub type InfoTable<T> = BinVec<NodeId, T>;

#[derive(Clone, Default)]
pub struct SemanticInfo {
    types: Vec<(NodeId, Type)>,
    function_signatures: Vec<(NodeId, FunctionSignature)>,
    resolved_symbols: Vec<(NodeId, NodeId)>,
}
impl SemanticInfo {
    pub fn add_type(&mut self, id: NodeId, ty: Type) {
        self.types.push((id, ty));
    }

    pub fn get_type(&self, id: &NodeId) -> Option<&Type> {
        util::slice_get_pair(&self.types, id)
    }

    pub fn add_resolved_symbol(&mut self, symbol: NodeId, resolved: NodeId) {
        self.resolved_symbols.push((symbol, resolved));
    }

    pub fn get_resolved_symbol(&self, id: &NodeId) -> Option<&NodeId> {
        util::slice_get_pair(&self.resolved_symbols, id)
    }

    pub fn get_function_signature(&self, id: &NodeId) -> Option<&FunctionSignature> {
        util::slice_get_pair(&self.function_signatures, id)
    }

    pub fn add_function_signature(&mut self, id: NodeId, function_signature: FunctionSignature) {
        self.function_signatures.push((id, function_signature));
    }

    pub fn add_function_params(&mut self, params: &[Parameter]) {
        for param in params {
            self.add_type(param.id, param.ty.clone());
        }
    }

    #[allow(unused)]
    fn insert<T>(v: &mut Vec<Option<T>>, index: usize, value: T) {
        if index >= v.len() {
            v.resize_with(index + 1, || None);
        }

        v[index] = Some(value);
    }
}
