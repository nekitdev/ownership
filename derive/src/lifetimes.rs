use std::collections::{HashMap, HashSet};

use syn::{GenericParam, Generics, Lifetime, WherePredicate};

use crate::by::By;

pub type LifetimeRef<'g> = &'g Lifetime;

pub type Outlived<'g> = HashMap<LifetimeRef<'g>, HashSet<LifetimeRef<'g>>>;
pub type Promoted<'g> = HashSet<LifetimeRef<'g>>;

pub fn outlived(generics: &Generics) -> Outlived<'_> {
    let mut graph = Outlived::new();

    // firstly, collect lifetimes from generic parameters

    generics.params.iter().for_each(|parameter| {
        if let GenericParam::Lifetime(lifetime_parameter) = parameter {
            let lifetime_ref = lifetime_parameter.lifetime.by_ref();

            lifetime_parameter.bounds.iter().for_each(|bound| {
                graph.entry(bound).or_default().insert(lifetime_ref);
            });
        }
    });

    // secondly, collect lifetimes from `where` clauses, in case there is one

    if let Some(ref where_clause) = generics.where_clause {
        where_clause.predicates.iter().for_each(|predicate| {
            if let WherePredicate::Lifetime(lifetime_predicate) = predicate {
                let lifetime_ref = lifetime_predicate.lifetime.by_ref();

                lifetime_predicate.bounds.iter().for_each(|bound| {
                    graph.entry(bound).or_default().insert(lifetime_ref);
                });
            }
        });
    }

    graph
}

pub fn promoted<'g, S: IntoIterator<Item = LifetimeRef<'g>>>(
    graph: &Outlived<'g>,
    start: S,
) -> Promoted<'g> {
    let mut promoted = Promoted::new();

    let mut stack: Vec<_> = start.into_iter().collect();

    while let Some(lifetime) = stack.pop() {
        #[allow(clippy::collapsible_if)]
        if promoted.insert(lifetime) {
            if let Some(outlived) = graph.get(lifetime) {
                stack.extend(outlived);
            }
        }
    }

    promoted
}
