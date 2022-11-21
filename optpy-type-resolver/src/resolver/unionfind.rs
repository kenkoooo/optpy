use std::collections::BTreeMap;

pub struct UnionFind<T> {
    parent: BTreeMap<T, T>,
    sizes: BTreeMap<T, usize>,
}

impl<T> UnionFind<T> {
    pub fn new() -> Self {
        Self {
            parent: BTreeMap::new(),
            sizes: BTreeMap::new(),
        }
    }
}
impl<T: Ord + Clone> UnionFind<T> {
    pub fn find(&mut self, x: &T) -> T {
        match self.parent.get(x).cloned() {
            Some(px) => {
                if x == &px {
                    px
                } else {
                    let px = self.find(&px);
                    self.parent.insert(x.clone(), px.clone());
                    px
                }
            }
            None => x.clone(),
        }
    }

    pub fn unite(&mut self, x: &T, y: &T) -> bool {
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
