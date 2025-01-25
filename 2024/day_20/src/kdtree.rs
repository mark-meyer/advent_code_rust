pub mod kdtree {

    pub struct KdIterator<T, const D: usize>  {
        pub current_node: KdTreeNode<T, D> 
    }

    #[derive(Debug)]
    pub struct KdTree<T, const D: usize> {
        pub root:Option<Box<KdTreeNode<T, D>>>
    }
    
    #[derive(Debug)]
    pub struct KdTreeNode<T, const D: usize> {
        value: [T; D],
        left: Option<Box<KdTreeNode<T,D>>>,
        right: Option<Box<KdTreeNode<T,D>>>
    }
    
    impl<T, const D: usize> KdTree<T, D> 
        where  T: PartialOrd + Copy + std::fmt::Display
    {
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
            *current = Some(
                Box::new(KdTreeNode{
                    value,
                    left:None,
                    right:None
                }))
        }
    
        pub fn values(&self) -> Vec<[T;D]> {
            let mut stack = vec![&self.root];
            let mut res = vec![];
            while !stack.is_empty() {
                match stack.pop().unwrap() {
                    Some(next) => {
                        res.push(next.value);
                        stack.push(&next.left);
                        stack.push(&next.right);
                    },
                    None => {}
                }
            }
            res
        }
    
        pub fn find(&self, value: [T; D]) -> Option<&KdTreeNode<T, D>> {
            let mut current = &self.root;
            let mut d = 0;
            while let Some(next) = current {
                if next.value == value {
                    return Some(&next)
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

        pub fn range_query(&self, min: [T; D], max: [T; D]) -> Vec<[T; D]> {
            let mut result = Vec::new();
            let mut stack = vec![(&self.root, 0)]; 
        
            while let (Some(node), d) = stack.pop().unwrap() {
                let mut in_range = true;
                for i in 0..D {
                    if node.value[i] < min[i] || node.value[i] > (max[i]) {
                        in_range = false;
                        break;
                    }
                }
                if in_range {                    
                    result.push(node.value);
                }
    
                if min[d] <= node.value[d] {
                    stack.push((&node.left, (d + 1) % D));
                }
                if max[d] >= node.value[d] {
                    stack.push((&node.right, (d + 1) % D));
                }
            }
            result
        }
    } 
    
    #[cfg(test)]
    mod test {
        use super::*;
    
        #[test]
        fn test_kd_new() {
            let mut kd = KdTree{root:None};
            kd.insert([2, 3]);
            kd.insert([4, 4]);
            kd.insert([3, 2]);
            assert_eq!(&kd.root.as_ref().unwrap().value, &[2, 3] );
            assert_eq!(&kd.root.unwrap().right.unwrap().left.unwrap().value, &[3, 2])
        }
        #[test]
        fn test_kd_range() {
            let mut kd = KdTree{root:None};
            kd.insert([6, -4, 2]);
        
            let res = kd.range_query([2, -4, -10], [6,0, 20]);
            assert_eq!(res, vec![[6,-4, 2]])
        }
        #[test]
        fn test_values() {
            let mut kd = KdTree{root:None};
            kd.insert([2, 3]);
            kd.insert([4, 4]);
            kd.insert([3, 2]);
            let res = kd.values();
            assert_eq!(res, [[2,3], [4,4], [3,2]])
        
        }
    }
}