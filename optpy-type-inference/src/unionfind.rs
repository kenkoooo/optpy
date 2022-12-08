use std::{collections::BTreeMap, fmt::Debug};

pub(crate) struct UnionFind<T> {
    pub(crate) parent: BTreeMap<T, T>,
    pub(crate) sizes: BTreeMap<T, usize>,
}

impl<T> Default for UnionFind<T> {
    fn default() -> Self {
        Self {
            parent: Default::default(),
            sizes: Default::default(),
        }
    }
}

impl<T> UnionFind<T>
where
    T: Clone + Ord + PartialEq + Debug,
{
    pub(crate) fn find(&mut self, x: &T) -> T {
        match self.parent.get(x).cloned() {
            Some(px) => {
                let px = self.find(&px);
                assert_ne!(&px, x);
                self.parent.insert(x.clone(), px.clone());
                px
            }
            None => x.clone(),
        }
    }

    pub(crate) fn unite(&mut self, x: &T, y: &T) -> bool {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            return false;
        }

        let size_x = self.sizes.get(&parent_x).cloned().unwrap_or(1);
        let size_y = self.sizes.get(&parent_y).cloned().unwrap_or(1);

        let (large, small) = if size_x < size_y {
            (parent_y, parent_x)
        } else {
            (parent_x, parent_y)
        };

        self.parent.insert(small, large.clone());
        self.sizes.insert(large, size_x + size_y);
        true
    }
}
