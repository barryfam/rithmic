use crate::BVec;
use super::*;

#[derive(Clone, Copy, Debug)]
pub struct DfsStep<E>
where E: Copy
{
    pub kind: DfsStepKind,
    pub p: usize,
    pub u: usize,
    pub edge: Option<E>,
    pub depth: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DfsStepKind {
    TreeNodeStart,
    TreeNodeFinish,
    BackEdge,
    ForwardEdge,
    CrossEdge,
}

impl<E, const FLAGS: usize> Graph<E, FLAGS>
where E: Copy
{
    pub fn dfs(&self, s: impl IntoIterator<Item=usize>) -> Vec<DfsStep<E>>
    {
        assert!(has_flags(FLAGS, SIMPLE) || !has_flags(FLAGS, UNDIRECTED),
            "Non-simple undirected graphs are not supported due to difficulty identifying parent edges");

        let mut dfs = vec![];

        enum Visit<E> {
            Start(usize, usize, Option<E>),
            Finish(usize, usize, Option<E>)
        }
        use Visit::*;
        let mut stack: Vec<Visit<E>> = s.into_iter().map(|s| Start(NONE, s, None)).collect();
        stack.reverse();

        // Edge type of (p, u):                     dir     undir   dag     forest/tree
        // tree = u not started                     *       * ⁽¹⁾   *       * ⁽¹⁾
        // back = u started, not finished           *       *
        // forward = u finished, after p start      *       ⁽²⁾     *
        // cross = u finished, before p start       *               *
        //
        // ¹ avoid traversing (u, p) after (p, u)
        // ² ignore "false-forward-edge" (p, u) because (u, p) has already been counted as a back edge
        //
        // Req. data structures:
        // start (bool)                                     *
        // start (time)                             *               *
        // finish (bool)                            *       *

        let n = self.size();

        let mut started = Some(BVec::new(n));
        let mut start_time = Some(vec![!0; n]);
        let mut finished = Some(BVec::new(n));

        match (has_flags(FLAGS, UNDIRECTED), has_flags(FLAGS, ACYCLIC)) {
            (false, false) => { started = None;                                     }
            (true , false) => {                 start_time = None;                  }
            (false, true ) => { started = None;                    finished = None; }
            (true , true ) => { started = None; start_time = None; finished = None; }
        }

        use DfsStepKind::*;
        let mut depth = 0;
        while let Some(visit) = stack.pop() {
            let time = dfs.len();
            match visit {
                Start(p, u, e) =>
                {
                    // DFS Start or Tree Edge
                    if has_flags(FLAGS, FOREST)
                        || started.as_ref().map_or(false, |started| !started[u])
                        || start_time.as_ref().map_or(false, |start_time| start_time[u] == !0)
                    {
                        started.as_mut().map(|started| started.set(u, true));
                        start_time.as_mut().map(|start_time| start_time[u] = time);

                        dfs.push(DfsStep{kind: TreeNodeStart, p, u, edge: e, depth});

                        stack.push(Finish(p, u, e));

                        for &(v, e) in &self.adj[u] {
                            if has_flags(FLAGS, UNDIRECTED) && v == p { continue }
                            stack.push(Start(u, v, Some(e)));
                        }

                        depth += 1;
                    }

                    // DFS Start on already visited node
                    else if p == NONE {
                        continue
                    }

                    // Back Edge
                    else if finished.as_ref().map_or(false, |finished| !finished[u])
                    {
                        dfs.push(DfsStep{kind: BackEdge, p, u, edge: e, depth});
                    }

                    // Forward / Cross Edge
                    else if !has_flags(FLAGS, UNDIRECTED)
                    {
                        let start_time = start_time.as_ref().unwrap();
                        if start_time[u] > start_time[p] {
                            dfs.push(DfsStep{kind: ForwardEdge, p, u, edge: e, depth});
                        }
                        else {
                            dfs.push(DfsStep{kind: CrossEdge, p, u, edge: e, depth});
                        }
                    }

                    // Discard "false-forward" in an undirected graph cycle
                    else if has_flags(FLAGS, UNDIRECTED) {}

                    else { debug_assert!(false) }
                }
                Finish(p, u, e) =>
                {
                    depth -= 1;
                    finished.as_mut().map(|finished| finished.set(u, true));
                    dfs.push(DfsStep{kind: TreeNodeFinish, p, u, edge: e, depth})
                }
            }
        }

        dfs
    }
}
