
pub struct KdIterator<'a, T, const D: usize> {
    pub stack: Vec<&'a Option<Box<KdTreeNode<T, D>>>>,
}
impl<'a, T, const D: usize> Iterator for KdIterator<'a, T, D> {
    type Item = &'a [T; D];

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            if let Some(node) = self.stack.pop().unwrap() {
                self.stack.push(&node.left);
                self.stack.push(&node.right);
                return Some(&node.value);
            }
        }
        None
    }
}

pub struct KdRangeIterator<'a, T: PartialOrd + Ord, const D: usize> {
    stack: Vec<(&'a Option<Box<KdTreeNode<T, D>>>, usize)>,
    min: [T; D],
    max: [T; D],
}
impl<'a, T, const D: usize> Iterator for KdRangeIterator<'a, T, D>
where
    T: PartialOrd + Ord,
{
    type Item = &'a [T; D];

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            if let (Some(node), d) = self.stack.pop().unwrap() {
                let mut in_range = true;
                for i in 0..D {
                    if node.value[i] < self.min[i] || node.value[i] > self.max[i] {
                        in_range = false;
                        break;
                    }
                }

                if self.min[d] <= node.value[d] {
                    self.stack.push((&node.left, (d + 1) % D));
                }
                if self.max[d] >= node.value[d] {
                    self.stack.push((&node.right, (d + 1) % D));
                }
                if in_range {
                    return Some(&node.value);
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct KdTree<T, const D: usize> {
    pub root: Option<Box<KdTreeNode<T, D>>>,
}

#[derive(Debug)]
pub struct KdTreeNode<T, const D: usize> {
    value: [T; D],
    left: Option<Box<KdTreeNode<T, D>>>,
    right: Option<Box<KdTreeNode<T, D>>>,
}

impl<T, const D: usize> KdTree<T, D>
where
    T: PartialOrd + Copy + Ord + std::fmt::Display,
{
    pub fn iter(&self) -> KdIterator<T, D> {
        KdIterator {
            stack: vec![&self.root],
        }
    }

    pub fn insert(&mut self, value: [T; D]) {
        let mut d = 0;
        let mut current = &mut self.root;
        while let Some(next) = current {
            if value[d] < next.value[d] {
                current = &mut next.left;
            } else {
                current = &mut next.right;
            }
            d = (d + 1) % D;
        }
        *current = Some(Box::new(KdTreeNode {
            value,
            left: None,
            right: None,
        }))
    }

    pub fn values(&self) -> Vec<[T; D]> {
        let mut stack = vec![&self.root];
        let mut res = vec![];
        while !stack.is_empty() {
            if let Some(next) = stack.pop().unwrap() {
                res.push(next.value);
                stack.push(&next.left);
                stack.push(&next.right);
            }
        }
        res
    }

    pub fn find(&self, value: [T; D]) -> Option<&KdTreeNode<T, D>> {
        let mut current = &self.root;
        let mut d = 0;
        while let Some(next) = current {
            if next.value == value {
                return Some(next);
            }
            if value[d] < next.value[d] {
                current = &next.left;
            } else {
                current = &next.right;
            }
            d = (d + 1) % D;
        }
        None
    }

    pub fn range_query(&self, min: [T; D], max: [T; D]) -> KdRangeIterator<T, D> {
        KdRangeIterator {
            stack: vec![(&self.root, 0)],
            min,
            max,
        }
    }
}
impl<T, const D: usize> From<Vec<[T; D]>> for KdTree<T, D>
where
    T: PartialOrd + Ord + Copy + std::fmt::Display,
{
    fn from(mut nodes: Vec<[T; D]>) -> Self {
        let mut stack = vec![(&mut nodes[..], 0)];
        let mut tree = KdTree { root: None };
        while let Some((slice, d)) = stack.pop() {
            if slice.is_empty() {
                continue;
            }
            slice.sort_by_key(|point| point[d]);
            let mid = slice.len() / 2;

            tree.insert(slice[mid]);
            let (min, max) = slice.split_at_mut(mid);
            stack.push((min, (d + 1) % D));
            stack.push((&mut max[1..], (d + 1) % D));
        }
        tree
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kd_new() {
        let mut kd = KdTree { root: None };
        kd.insert([2, 3]);
        kd.insert([4, 4]);
        kd.insert([3, 2]);
        assert_eq!(&kd.root.as_ref().unwrap().value, &[2, 3]);
        assert_eq!(
            &kd.root.unwrap().right.unwrap().left.unwrap().value,
            &[3, 2]
        )
    }
    #[test]
    fn test_kd_range() {
        let mut kd = KdTree { root: None };
        kd.insert([6, -4, 2]);

        let mut res = kd.range_query([2, -4, -10], [6, 0, 20]);
        assert_eq!(res.next(), Some(&[6, -4, 2]))
    }
    #[test]
    fn test_values() {
        let mut kd = KdTree { root: None };
        kd.insert([2, 3]);
        kd.insert([4, 4]);
        kd.insert([3, 2]);
        let res = kd.values();
        assert_eq!(res, [[2, 3], [4, 4], [3, 2]])
    }
}
