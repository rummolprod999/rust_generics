trait Graph<N, E> {
    fn i_has_edge(&self, _: &N, _: &N) -> bool;
    fn i_edges(&self, _: &N) -> Vec<E>;
    // etc
}

trait GraphAssoc {
    type N;
    type E;

    fn has_edge(&self, _: &Self::N, _: &Self::N) -> bool;
    fn edges(&self, _: &Self::N) -> Vec<Self::E>;
}

struct Test<T, G, L>(T, G, L);

impl<T, G, L> GraphAssoc for Test<T, G, L> {
    type N = G;
    type E = L;

    fn has_edge(&self, n1: &G, n2: &G) -> bool {
        true
    }

    fn edges(&self, n: &G) -> Vec<L> {
        Vec::new()
    }
}

impl<T, G, L> Graph<T, G> for Test<T, G, L> {
    fn i_has_edge(&self, n1: &T, n2: &T) -> bool {
        true
    }

    fn i_edges(&self, n: &T) -> Vec<G> {
        Vec::new()
    }
}

fn i_distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
    32
}

fn distance<G: GraphAssoc>(graph: &G, start: &G::N, end: &G::N) -> u32 { 32 }

fn main() {
    println!("Hello, world!");
    let graph_assoc = Test(1, 2, 3);
    println!("{}", graph_assoc.has_edge(&1, &2));
    println!("{}", graph_assoc.i_has_edge(&1, &2))
}
