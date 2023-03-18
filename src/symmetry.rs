use std::fmt::Display;

/// A struct to represent the orbit-fixing conditions of a graph
/// Both are vertex indices with the expectation that the first
/// is the smaller index (i.e. smaller depth).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Condition {
    pub u: usize,
    pub v: usize,
    pub max: usize,
}
impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}<{}", self.u, self.v)
    }
}
impl Condition {
    pub fn new(u: usize, v: usize) -> Self {
        Condition {
            u,
            v,
            max: u.max(v),
        }
    }

    pub fn is_respected(&self, u: usize, v: usize) -> bool {
        if u == self.u && v == self.v {
            return u < v
        }
        true
    }
}

// pub type Conditions = Vec<Condition>;
#[derive(Clone, Debug)]
pub struct Conditions {
    conditions: Vec<Condition>
}
impl Conditions {
    pub fn from_vec(conditions: Vec<Condition>) -> Self {
        Conditions {
            conditions
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&Condition> {
        self.conditions.iter()
    }

    pub fn contains(&self, c: &Condition) -> bool {
        self.conditions.contains(c)
    }

    pub fn is_empty(&self) -> bool {
        self.conditions.is_empty()
    }

    pub fn retain(&mut self, f: impl Fn(&Condition) -> bool) {
        self.conditions.retain(f)
    }

    pub fn respects_all(&self, u: usize, v: usize) -> bool {
        self.conditions.iter().all(|c| c.is_respected(u, v))
    }
}

