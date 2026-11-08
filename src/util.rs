use crate::tokenize::Token;

#[allow(unused)]
fn print_tokens(tokens: &Vec<Token>, source: &str) {
    for token in tokens {
        println!("token '{}' at {}", token.lexeme(source), token.span.start())
    }
}

pub fn vec_insert_with_padding<T: Default>(v: &mut Vec<T>, index: usize, value: T) {
    if index >= v.len() {
        v.resize_with(index + 1, T::default);
    }

    v[index] = value;
}

#[derive(Clone, Default)]
pub struct BinVec<I, T> {
    info: Vec<(I, T)>,
}

impl<I: Ord + Clone + Copy, T> BinVec<I, T> {
    pub fn push(&mut self, id: I, val: T) {
        self.info.push((id, val));
    }
    pub fn get(&self, id: &I) -> Option<&T> {
        self.info
            .binary_search_by_key(id, |entry| entry.0)
            .ok()
            .map(|index| &self.info[index].1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_insert_with_padding_simple() {
        let mut v = vec![1, 2, 3];
        vec_insert_with_padding(&mut v, 5, 5);

        assert_eq!(v, vec![1, 2, 3, 0, 0, 5]);
    }
}
