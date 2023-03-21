use std::fmt::Display;

/// A struct to represent the orbit-fixing conditions of a graph
/// Both are vertex indices with the expectation that the first
/// is the smaller index (i.e. smaller depth).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Condition {
    pub u: usize,
    pub v: usize,
}
impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}<{}", self.u, self.v)
    }
}
impl Condition {
    pub fn new(u: usize, v: usize) -> Self {
        assert!(u < v);
        Condition { u, v }
    }

    /// Returns true if the condition is respected by the given graph
    ///
    /// # Arguments
    /// * `d1` - The depth of the first vertex
    /// * `d2` - The depth of the second vertex
    /// * `u` - The first vertex
    /// * `v` - The second vertex
    pub fn is_respected(&self, d1: usize, d2: usize, u: usize, v: usize) -> bool {
        if d1 == self.u && d2 == self.v {
            return u < v;
        }
        true
    }

    pub fn max(&self) -> usize {
        self.v
    }

    pub fn min(&self) -> usize {
        self.u
    }
}

// pub type Conditions = Vec<Condition>;
#[derive(Clone, Debug)]
pub struct Conditions {
    conditions: Vec<Condition>,
}
impl Display for Conditions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("|");
        for (idx, c) in self.conditions.iter().enumerate() {
            if idx > 0 {
                s.push_str(" ");
            }
            s.push_str(&format!("{}", c));
        }
        s.push_str("|");
        write!(f, "{}", s)
    }
}
impl Conditions {
    pub fn from_vec(conditions: Vec<Condition>) -> Self {
        Conditions { conditions }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Condition> {
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

    /// Returns true if the conditions are respected for the given pair of vertices
    ///
    /// # Arguments
    /// * 'd1' - The depth of the first vertex
    /// * 'd2' - The depth of the second vertex
    /// * 'u' - The first vertex index
    /// * 'v' - The second vertex index
    #[allow(dead_code)]
    pub fn respects_all(&self, d1: usize, d2: usize, u: usize, v: usize) -> bool {
        assert!(d1 <= d2, "d1 must be less than or equal to d2");
        self.conditions.iter().all(|c| c.is_respected(d1, d2, u, v))
    }

    /// Returns true if any of the conditions are respected for the given pair of vertices
    ///
    /// # Arguments
    /// * 'd1' - The depth of the first vertex
    /// * 'd2' - The depth of the second vertex
    /// * 'u' - The first vertex index
    /// * 'v' - The second vertex index
    pub fn respects_any(&self, d1: usize, d2: usize, u: usize, v: usize) -> bool {
        assert!(d1 <= d2, "d1 must be less than or equal to d2");
        self.conditions.iter().any(|c| c.is_respected(d1, d2, u, v))
    }
}

#[cfg(test)]
mod testing {

    use super::*;

    #[test]
    fn condition_a() {
        let c1 = Condition::new(0, 1);
        let conditions = Conditions::from_vec(vec![c1]);

        let test_set = vec![
            (0, 1, 10, 20, true),  // should pass
            (0, 1, 20, 10, false), // automorphism, should fail
            (1, 2, 20, 10, true),  // irrelevant, should pass
            (1, 2, 20, 10, true),  // irrelevant, should pass
        ];
        for (d1, d2, u, v, expected) in test_set {
            assert_eq!(conditions.respects_all(d1, d2, u, v), expected);
        }
    }

    #[test]
    fn condition_b() {
        let c1 = Condition::new(0, 1);
        let c2 = Condition::new(1, 2);
        let conditions = Conditions::from_vec(vec![c1, c2]);

        let test_set = vec![
            (0, 1, 10, 20, true),  // fixed-orbit, should pass
            (0, 1, 20, 10, false), // automorphism, should fail
            (1, 2, 30, 40, true),  // fixed-orbit, should pass
            (1, 2, 40, 30, false), // automorphism, should pass
        ];
        for (d1, d2, u, v, expected) in test_set {
            assert_eq!(conditions.respects_all(d1, d2, u, v), expected);
        }
    }

    #[test]
    fn condition_c() {
        let c1 = Condition::new(0, 1);
        let c2 = Condition::new(2, 3);
        let conditions = Conditions::from_vec(vec![c1, c2]);

        let test_set = vec![
            (0, 1, 10, 20, true),  // fixed-orbit, should pass
            (0, 1, 20, 10, false), // automorphism, should fail
            (1, 2, 30, 40, true),  // irrelevant, should pass
            (1, 2, 40, 30, true),  // irrelevant, should pass
            (2, 3, 50, 60, true),  // fixed-orbit, should pass
            (2, 3, 60, 50, false), // automorphism, should fail
            (3, 4, 70, 80, true),  // irrelevant, should pass
            (3, 4, 80, 70, true),  // irrelevant, should pass
        ];
        for (d1, d2, u, v, expected) in test_set {
            println!("{}<{}: {}<{}: {}", d1, d2, u, v, expected);
            assert_eq!(conditions.respects_all(d1, d2, u, v), expected);
        }
    }
}
