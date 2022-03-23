use crate::logic::Ident;
use crate::parser::Rule;
use pest::iterators::Pair;

#[derive(Debug, Clone, PartialEq)]
/// The smallest (atomic) operand in a logical formula.
pub enum Atom {
    /// A boolean value, either `true` or `false`
    Boolean(bool),
    Predicate(Predicate),
    Unknown(Ident),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Predicate {
    name: Ident,
    args: Vec<Var>,
}

#[derive(Debug, Clone, PartialEq)]
/// Objects that can be passed as arguments to predicates
pub enum Var {
    Fixed(Ident),
    Anonymous(Ident),
}

impl Atom {
    pub fn from_pair(pair: Pair<Rule>) -> Self {
        assert_eq!(
            std::mem::discriminant(&Rule::Atom),
            std::mem::discriminant(&pair.as_rule())
        );

        let child = pair.into_inner().next().unwrap();
        match child.as_rule() {
            Rule::Predicate => {
                let mut idents = child.into_inner().map(|p| p.as_str().to_string());
                Self::Predicate(Predicate {
                    name: idents.next().unwrap(),
                    args: idents.map(|s| Var::Fixed(s)).collect(),
                })
            }
            Rule::Boolean => match child.as_str() {
                "true" => Self::Boolean(true),
                "false" => Self::Boolean(false),
                _ => unreachable!(),
            },
            Rule::Unknown => Self::Unknown(child.as_str().to_string()),
            _ => unreachable!("Converting non-atom value to atom"),
        }
    }
}

impl Predicate {
    pub fn replace(&mut self, to_replace: &Var, replace_with: &Var) {
        for arg in &mut self.args {
            if arg == to_replace {
                *arg = replace_with.clone();
            }
        }
    }

    /// Other is assumed to not contain any anonymous variables
    pub fn match_args(&self, other: &Self) -> Option<Vec<usize>> {
        // Predicate names must be the same
        if self.name != other.name {
            return None;
        }

        // indices of anonymous args that now take the value of other
        let mut captured_args = vec![];
        let matches = self
            .args
            .iter()
            .zip(&other.args)
            .enumerate()
            .all(|(ix, (a1, a2))| {
                if let Var::Anonymous(_) = a1 {
                    captured_args.push(ix);
                    return true;
                } else {
                    return a1 == a2;
                }
            });

        if matches {
            return Some(captured_args);
        } else {
            return None;
        }
    }
}