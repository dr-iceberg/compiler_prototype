use crate::grammar::{Function, Stmt};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub usize);

impl NodeId {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

#[derive(Default)]
pub struct AstMap<'ast> {
    functions: Vec<&'ast Function>,
    #[allow(unused)]
    statements: Vec<&'ast Stmt>,
}

impl<'ast> AstMap<'ast> {
    pub fn add_function(&mut self, function: &'ast Function) {
        self.functions.push(function);
    }
    pub fn get_function(&self, id: &NodeId) -> Option<&Function> {
        self.functions.iter().find(|func| func.id == *id).copied()
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use crate::grammar::Type;

    use super::*;

    #[test]
    fn ast_map_test() {
        let id = NodeId(0);
        let name = "name";
        let f = Function {
            id,
            name: String::from(name),
            parameters: Vec::new(),
            body: Vec::new(),
            return_type: Type::Unknown,
        };
        let mut ast_map = AstMap::default();
        ast_map.add_function(&f);

        let reference = ast_map.get_function(&id);
        assert_matches!(reference, Some(_));
        if let Some(func) = reference {
            assert_eq!(func.id, id);
            assert_eq!(func.name.as_str(), name);
        }
    }
}
