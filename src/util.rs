use crate::tokenize::Token;

#[allow(unused)]
fn print_tokens(tokens: &Vec<Token>, source: &str) {
    for token in tokens {
        println!("token '{}' at {}", token.lexeme(source), token.span.start())
    }
}

#[allow(unused)]
pub fn vec_insert_with_padding<T: Default>(v: &mut Vec<T>, index: usize, value: T) {
    if index >= v.len() {
        v.resize_with(index + 1, T::default);
    }

    v[index] = value;
}
/// A vector wrapper that uses binary search to get an
/// element `T` by the specified id `I`.
///
/// Its used as a balance between
#[derive(Debug, Clone, Default)]
pub struct BinVec<I, T> {
    info: Vec<(I, T)>,
}

impl<I, T> BinVec<I, T>
where
    I: Ord + Clone + Copy,
    T: PartialEq,
{
    pub fn new() -> Self {
        Self { info: Vec::new() }
    }
    pub fn push(&mut self, id: I, val: T) {
        self.info.push((id, val));
    }
    /// Retrieve the value with binary search
    pub fn get(&self, id: &I) -> Option<&T> {
        self.info
            .binary_search_by_key(id, |entry| entry.0)
            .ok()
            .map(|index| &self.info[index].1)
    }

    /// Get the id given the value
    pub fn get_id(&self, val: &T) -> Option<&I> {
        for (i, v) in &self.info {
            if v == val {
                return Some(i);
            }
        }
        None
    }

    /// Sort the elements with `I` as a key
    pub fn sort(&mut self) {
        self.info.sort_unstable_by_key(|entry| entry.0);
    }

    pub fn elements(&self) -> &[(I, T)] {
        &self.info
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

    #[test]
    fn bin_vec_test() {
        let mut bv = BinVec::<usize, usize>::default();
        let vals: Vec<(usize, usize)> = vec![(20, 0), (1, 2), (7, 3), (5, 8), (0, 0)];
        for val in &vals {
            bv.push(val.0, val.1);
        }

        assert_eq!(bv.elements(), vals.as_slice());

        bv.sort();

        assert!(bv.elements().is_sorted_by_key(|entry| entry.0));

        assert_eq!(bv.get(&7).unwrap(), &3);
        assert_eq!(bv.get(&20).unwrap(), &0);
        assert_eq!(bv.get(&0).unwrap(), &0);
        assert_eq!(bv.get(&5).unwrap(), &8);
        assert_eq!(bv.get(&1).unwrap(), &2);
    }
}
