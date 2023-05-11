struct WeightedQuickUnionUF {
    id: Vec<usize>,
    sz: Vec<usize>,
}

impl WeightedQuickUnionUF {
    fn weighted_quick_union(&mut self, n: usize) -> () {
        for i in 0..n {
            self.id.push(i);
            self.sz.push(1);
        }
    }

    fn root(&mut self, mut idx: usize) -> usize {
        while idx != self.id[idx] {
            self.id[idx] = self.id[self.id[idx]];
            idx = self.id[idx];
        }
        return idx;
    }

    fn connected(&mut self, p: usize, q: usize) -> bool {
        return self.root(p) == self.root(q);
    }

    fn union(&mut self, p: usize, q: usize) -> () {
        let i = self.root(p);
        let j = self.root(q);
        if i == j {
            return;
        }
        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
    }
}

fn main() {
    let mut uf = WeightedQuickUnionUF { id: vec![], sz: vec![] };

    uf.weighted_quick_union(10);
    uf.union(0, 1);
    uf.union(1, 8);
    uf.union(8, 3);
    uf.union(4, 1);
    uf.union(2, 7);
    println!("{:?}", uf.id);
    println!("{}", uf.connected(0, 2));
    println!("{}", uf.connected(0, 4));
}
