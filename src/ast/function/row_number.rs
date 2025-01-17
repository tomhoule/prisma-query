use crate::ast::{Column, IntoOrderDefinition, Over};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RowNumber<'a> {
    pub(crate) over: Over<'a>,
}

impl<'a> RowNumber<'a> {
    /// Define the order of the row number. Is the row order if not set.
    pub fn order_by<T>(mut self, value: T) -> Self
    where
        T: IntoOrderDefinition<'a>,
    {
        self.over.ordering = self.over.ordering.append(value.into_order_definition());
        self
    }

    /// Define the partitioning of the row number
    pub fn partition_by<T>(mut self, partition: T) -> Self
    where
        T: Into<Column<'a>>,
    {
        self.over.partitioning.push(partition.into());
        self
    }
}

/// A number from 1 to n in specified order
///
/// ```rust
/// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
/// let fun = Function::from(row_number().order_by("created_at").partition_by("name"));
///
/// let query = Select::from_table("users")
///     .column("id")
///     .value(fun.alias("num"));
///
/// let (sql, _) = Sqlite::build(query);
///
/// assert_eq!(
///     "SELECT `id`, ROW_NUMBER() OVER(PARTITION BY `name` ORDER BY `created_at`) AS `num` FROM `users`",
///     sql
/// );
/// ```
#[inline]
pub fn row_number<'a>() -> RowNumber<'a> {
    RowNumber::default()
}
