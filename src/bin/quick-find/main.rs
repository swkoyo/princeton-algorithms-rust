struct QuickFindUF {
    id: Vec<usize>,
}

impl QuickFindUF {
    fn quick_find(&mut self, n: usize) -> () {
        for i in 0..n {
            self.id.push(i);
        }
    }

    fn connected(&self, p: usize, q: usize) -> bool {
        return self.id[p] == self.id[q];
    }

    fn union(&mut self, p: usize, q: usize) -> () {
        let pid = self.id[p];
        let qid = self.id[q];

        for i in 0..self.id.len() {
            if self.id[i] == pid {
                self.id[i] = qid;
            }
        }
    }
}

fn main() {
    let mut uf = QuickFindUF {
        id: vec![],
    };

    uf.quick_find(10);
    uf.union(0, 1);
    uf.union(1, 8);
    println!("{:?}", uf.id);
    println!("{}", uf.connected(0, 2));
    println!("{}", uf.connected(0, 8));
}
