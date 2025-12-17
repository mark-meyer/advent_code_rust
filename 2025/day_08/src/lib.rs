use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<const D: usize> {
    pub coords: [i64; D],
    pub id: usize
}

impl<const D:usize> Point<D>  {
    pub fn square_distance(&self, other: &Point<D>) -> u64 {
        self.coords.iter().zip(other.coords.iter())
        .map(|(a, b)| a.abs_diff(*b).pow(2))
        .sum()
    }
}

pub struct BoundingBox<const D: usize> {
    pub min: [i64; D],
    pub max: [i64; D],
}

impl<const D: usize> BoundingBox<D> {
    pub fn from_points(points: &[Point<D>]) -> Self {
        if points.is_empty() {
            return BoundingBox { min: [0; D], max: [0; D] };
        }

        let mut min = [i64::MAX; D];
        let mut max = [i64::MIN; D];

        for p in points {
            for (i, v) in p.coords.iter().enumerate() {
                min[i] = min[i].min(*v);
                max[i] = max[i].max(*v);
            }
        }
        BoundingBox { min, max }
    }

    pub fn min_dist_sq(&self, other: &BoundingBox<D>) -> u64 {
        let mut dist: u64 = 0;
        for i in 0..D {
            let d = if other.max[i] < self.min[i] {
                (self.min[i] - other.max[i]) as u64
            } else if self.max[i] < other.min[i] {
                (other.min[i] - self.max[i]) as u64
            } else {
                0
            };
            dist += d * d
        }
        dist
    }

    // Distance from a single point to this box
    pub fn dist_sq_point(&self, p: &Point<D>) -> u64 {
        let mut dist_sq = 0;
        for i in 0..D {
            let v = p.coords[i];
            if v < self.min[i] {
                let d = (self.min[i] - v) as u64;
                dist_sq += d * d;
            } else if v > self.max[i] {
                let d = (v - self.max[i]) as u64;
                dist_sq += d * d;
            }
        }
        dist_sq
    }
}

pub struct KDTreeNode<const D: usize> {
    pub point: Point<D>,
    pub bounding_box: BoundingBox<D>,
    pub left: Option<Box<KDTreeNode<D>>>,
    pub right: Option<Box<KDTreeNode<D>>>
}

#[derive(Default)]
pub struct KDTree<const D: usize> {
    pub root: Option<Box<KDTreeNode<D>>>
}

impl<const D: usize> KDTree<D> {
    pub fn new(mut points: Vec<Point<D>>) -> Self {
        let root = Self::build_recursive(&mut points, 0);
        KDTree { root }
    }

    pub fn build_recursive(points: &mut [Point<D>], depth: usize) -> Option<Box<KDTreeNode<D>>> {
        if points.is_empty() {
            return None;
        }

        let dim = depth % D;
        let median_idx = points.len() / 2;

        points.select_nth_unstable_by_key(median_idx, |p| {p.coords[dim]});
            

        let bounding_box = BoundingBox::from_points(points);

        let (left_points, right_points_with_median) = points.split_at_mut(median_idx);
        let (median_slice, right_points) = right_points_with_median.split_at_mut(1);

        let point = median_slice[0];

        Some(Box::new(KDTreeNode {
            point,
            bounding_box,
            left: Self::build_recursive(left_points, depth+1),
            right: Self::build_recursive(right_points, depth+1)
        }))
    }
}

enum QueueItem<'a, const D: usize> {
    // store ids of points to help avoid dupes like (A, B) (B, A)
    PointPair(u64, &'a Point<D>, &'a Point<D>),
    NodeNode(u64, &'a KDTreeNode<D>, &'a KDTreeNode<D>),
    PointNode(u64, &'a Point<D>, &'a KDTreeNode<D>)
}

impl<'a, const D: usize> PartialEq for QueueItem<'a, D> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a, const D: usize> Eq for QueueItem<'a, D> {}

impl<'a, const D: usize> Ord for QueueItem<'a, D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let key_self = match self {
            QueueItem::PointPair(d, ..) => (*d, 0),
            QueueItem::PointNode(d, ..) => (*d, 1),
            QueueItem::NodeNode(d, ..)  => (*d, 2),
        };
        let key_other = match other {
            QueueItem::PointPair(d, ..) => (*d, 0),
            QueueItem::PointNode(d, ..) => (*d, 1),
            QueueItem::NodeNode(d, ..)  => (*d, 2),
        };
        key_other.cmp(&key_self)
    }
}

impl<'a, const D: usize> PartialOrd for QueueItem<'a, D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct ClosestPairIterator<'a, const D: usize> {
    heap: BinaryHeap<QueueItem<'a, D>>
}

impl<'a, const D: usize> ClosestPairIterator<'a, D> {
    pub fn new(tree: &'a KDTree<D>) -> Self {
        let mut heap = BinaryHeap::new();

        if let Some(root) = &tree.root {
            heap.push(QueueItem::NodeNode(0, root, root));
        }
        Self { heap }
    }

    fn size(bounding_box: &BoundingBox<D>) -> i64 {
        (0..D).fold(0, |a, idx| a + bounding_box.max[idx] - bounding_box.min[idx])
    }
}

impl<'a, const D: usize> Iterator for ClosestPairIterator<'a, D> {
    type Item = (u64, &'a Point<D>, &'a Point<D>);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.heap.pop() {
            match item {
                QueueItem::PointPair(d, p1, p2) => {
                    if p1.id <= p2.id {
                        return Some((d, p1, p2));
                    } else {
                        return Some((d, p2, p1));
                    }
                }

                QueueItem::PointNode(_, p, node) => {
                    // 1. Compare P vs Node.point
                    let d = p.square_distance(&node.point);
                    self.heap.push(QueueItem::PointPair(d, p, &node.point));

                    // 2. Compare P vs Node.left
                    if let Some(left) = &node.left {
                        let d_box = left.bounding_box.dist_sq_point(p);
                        self.heap.push(QueueItem::PointNode(d_box, p, left));
                    }
                    
                    if let Some(right) = &node.right {
                        let d_box = right.bounding_box.dist_sq_point(p);
                        self.heap.push(QueueItem::PointNode(d_box, p, right));
                    }
                }

                QueueItem::NodeNode(_, node_a, node_b) => {
                    if std::ptr::eq(node_a, node_b) {                        
                        if let Some(left) = &node_a.left {
                            let d = left.bounding_box.dist_sq_point(&node_a.point);
                            self.heap.push(QueueItem::PointNode(d, &node_a.point, left));
                            // Recurse Left vs Left
                            self.heap.push(QueueItem::NodeNode(0, left, left));
                        }
                        
                        if let Some(right) = &node_a.right {
                            let d = right.bounding_box.dist_sq_point(&node_a.point);
                            self.heap.push(QueueItem::PointNode(d, &node_a.point, right));
                            // Recurse Right vs Right
                            self.heap.push(QueueItem::NodeNode(0, right, right));
                        }

                        if let (Some(l), Some(r)) = (&node_a.left, &node_a.right) {
                            let d = l.bounding_box.min_dist_sq(&r.bounding_box);
                            self.heap.push(QueueItem::NodeNode(d, l, r));
                        }
                        
                    } else {
                        // Distinct Nodes Split the "larger" node and decompose.
                        
                        if Self::size(&node_a.bounding_box) > Self::size(&node_b.bounding_box) {
                            let d_p = node_b.bounding_box.dist_sq_point(&node_a.point);
                            self.heap.push(QueueItem::PointNode(d_p, &node_a.point, node_b));
                            

                            if let Some(left) = &node_a.left {
                                let d = left.bounding_box.min_dist_sq(&node_b.bounding_box);
                                self.heap.push(QueueItem::NodeNode(d, left, node_b));
                            }
                            if let Some(right) = &node_a.right {
                                let d = right.bounding_box.min_dist_sq(&node_b.bounding_box);
                                self.heap.push(QueueItem::NodeNode(d, right, node_b));
                            }
                        } else {

                            let d_p = node_a.bounding_box.dist_sq_point(&node_b.point);
                            self.heap.push(QueueItem::PointNode(d_p, &node_b.point, node_a));
                            
                            if let Some(left) = &node_b.left {
                                let d = left.bounding_box.min_dist_sq(&node_a.bounding_box);
                                self.heap.push(QueueItem::NodeNode(d, node_a, left));
                            }
                            if let Some(right) = &node_b.right {
                                let d = right.bounding_box.min_dist_sq(&node_a.bounding_box);
                                self.heap.push(QueueItem::NodeNode(d, node_a, right));
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

pub struct UnionFind {
    pub parent: Vec<usize>,
    pub sizes: Vec<usize>,
    pub num_components: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            sizes:vec![1;n],
            num_components: n
        }
    }
    pub fn find(&mut self, node: usize) -> usize {
        if self.parent[node] != node {
            let root = self.find(self.parent[node]);
            self.parent[node] = root;
        }
        self.parent[node]
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);

        if root_a == root_b {
            return false;
        }
        if self.sizes[root_a] < self.sizes[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }
        self.parent[root_b] = root_a;
        self.sizes[root_a] += self.sizes[root_b];
        self.num_components -= 1;
        true
    }

    pub fn get_component_size(&mut self, node: usize) -> usize {
        let root = self.find(node);
        self.sizes[root]
    }
}


pub fn parse_file<const D: usize>(f: File) -> Result<Vec<[i64; D]>, Box<dyn Error>> {
    BufReader::new(f)
    .lines()
    .map(|line| {
        let line = line?;
        let numbers:Vec<i64> = line.split(",")
        .map(|s| s.parse::<i64>())
        .collect::<Result<_, _>>()?;

        let coords: [i64; D] = numbers.try_into()
                .map_err(|v: Vec<i64>| {
                    format!("Dimension mismatch: expected {}, got {}", D, v.len())
                })?;
        Ok(coords)
    }).collect()
    
}

pub fn make_kd_tree<const D: usize>(coords: &Vec<[i64; D]>) -> KDTree<D> {
    let mut points = Vec::new();
    for (id, coord) in coords.iter().enumerate() {
        let p = Point{
            coords: *coord,
            id
        };
        points.push(p);
    }
    KDTree::new( points)
}

pub fn part_one(tree: &KDTree<3>, iterations: usize, num_points:usize) -> usize {
    let pairs = ClosestPairIterator::new(&tree);
    let mut uf: UnionFind = UnionFind::new(num_points);

    for (_dist, p1, p2) in pairs.take(iterations) {
        uf.union(p1.id, p2.id);
    }
    uf.sizes.iter().k_largest(3).product()
}

pub fn part_two(tree: &KDTree<3>, target_size: usize, num_points:usize) -> i64 {
    let pairs = ClosestPairIterator::new(&tree);
    let mut uf: UnionFind = UnionFind::new(num_points);

    for (_dist, p1, p2) in pairs {
        uf.union(p1.id, p2.id);
        if uf.num_components == 1  {
            let p = uf.find(p1.id);
            if target_size == uf.sizes[p] {
                    return p1.coords[0] * p2.coords[0];
            }
        }
    }
    panic!("No answer found")
}
