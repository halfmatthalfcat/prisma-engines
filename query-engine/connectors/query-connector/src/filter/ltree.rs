use crate::{Filter, LtreeCompare};
use prisma_models::{PrismaValue, ScalarField, ScalarFieldRef};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LtreeFilter {
    pub field: Arc<ScalarField>,
    pub condition: LtreeCondition,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LtreeCondition {
    IsAncestor(Vec<PrismaValue>),
    NotAncestor(Vec<PrismaValue>),
    IsDescendent(Vec<PrismaValue>),
    NotDescendent(Vec<PrismaValue>),
    Matches(Vec<PrismaValue>),
    NotMatches(Vec<PrismaValue>),
    FullTextMatches(PrismaValue),
    NotFullTextMatches(PrismaValue),
}

impl LtreeCompare for ScalarFieldRef {
    fn ltree_is_ancestor<T>(&self, values: Vec<T>) -> Filter
    where
        T: Into<PrismaValue>,
    {
        Filter::from(LtreeFilter {
            field: Arc::clone(self),
            condition: LtreeCondition::IsAncestor(values.into_iter().map(Into::into).collect()),
        })
    }

    fn ltree_not_ancestor<T>(&self, values: Vec<T>) -> Filter
    where
        T: Into<PrismaValue>,
    {
        Filter::from(LtreeFilter {
            field: Arc::clone(self),
            condition: LtreeCondition::NotAncestor(values.into_iter().map(Into::into).collect()),
        })
    }

    fn ltree_is_descendant<T>(&self, values: Vec<T>) -> Filter
    where
        T: Into<PrismaValue>,
    {
        Filter::from(LtreeFilter {
            field: Arc::clone(self),
            condition: LtreeCondition::IsDescendent(values.into_iter().map(Into::into).collect()),
        })
    }

    fn ltree_not_descendant<T>(&self, values: Vec<T>) -> Filter
    where
        T: Into<PrismaValue>,
    {
        Filter::from(LtreeFilter {
            field: Arc::clone(self),
            condition: LtreeCondition::NotDescendent(values.into_iter().map(Into::into).collect()),
        })
    }

    fn ltree_match<T>(&self, values: Vec<T>) -> Filter
    where
        T: Into<PrismaValue>,
    {
        Filter::from(LtreeFilter {
            field: Arc::clone(self),
            condition: LtreeCondition::Matches(values.into_iter().map(Into::into).collect()),
        })
    }

    fn ltree_not_match<T>(&self, values: Vec<T>) -> Filter
    where
        T: Into<PrismaValue>,
    {
        Filter::from(LtreeFilter {
            field: Arc::clone(self),
            condition: LtreeCondition::NotMatches(values.into_iter().map(Into::into).collect()),
        })
    }

    fn ltree_match_fulltext<T>(&self, value: T) -> Filter
    where
        T: Into<PrismaValue>,
    {
        Filter::from(LtreeFilter {
            field: Arc::clone(self),
            condition: LtreeCondition::FullTextMatches(value.into()),
        })
    }

    fn ltree_not_match_fulltext<T>(&self, value: T) -> Filter
    where
        T: Into<PrismaValue>,
    {
        Filter::from(LtreeFilter {
            field: Arc::clone(self),
            condition: LtreeCondition::NotFullTextMatches(value.into()),
        })
    }
}
