struct QuickUnionUF {
    id: Vec<usize>,
}

impl QuickUnionUF {
    fn quick_union(&mut self, n: usize) -> () {
        for i in 0..n {
            self.id.push(i);
        }
    }

    fn root(&self, mut idx: usize) -> usize {
        while idx != self.id[idx] {
            idx = self.id[idx];
        }
        return idx;
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        return self.root(p) == self.root(q);
    }

    fn union(&mut self, p: usize, q: usize) -> () {
        let i = self.root(p);
        let j = self.root(q);
        self.id[i] = j;
    }
}

fn main() {
    let mut uf = QuickUnionUF { id: vec![] };

    uf.quick_union(10);
    uf.union(0, 1);
    uf.union(1, 8);
    uf.union(8, 3);
    uf.union(4, 1);
    println!("{:?}", uf.id);
    println!("{}", uf.connected(0, 2));
    println!("{}", uf.connected(0, 4));
}
