use crate::analyzer::FunctionSignature;
use crate::ast::NodeId;
use crate::grammar::{Parameter, Type};
use crate::util::BinVec;

pub type InfoTable<T> = BinVec<NodeId, T>;

#[derive(Clone, Default)]
pub struct SemanticInfo {
    types: InfoTable<Type>,
    function_signatures: InfoTable<FunctionSignature>,
}
impl SemanticInfo {
    pub fn add_type(&mut self, id: NodeId, ty: Type) {
        self.types.push(id, ty);
    }

    pub fn get_function_signature(&self, id: &NodeId) -> Option<&FunctionSignature> {
        self.function_signatures.get(id)?.as_ref()
    }

    pub fn add_function_signature(&mut self, id: NodeId, function_signature: FunctionSignature) {
        self.function_signatures.push(id, function_signature);
    }

    pub fn add_function_params(&mut self, params: &[Parameter]) {
        for param in params {
            self.types.push(param.id, param.ty.clone());
        }
    }

    fn insert<T>(v: &mut Vec<Option<T>>, index: usize, value: T) {
        if index >= v.len() {
            v.resize_with(index + 1, || None);
        }

        v[index] = Some(value);
    }
}
