use crate::ast::{Column, ConditionTree, DatabaseValue, Expression};
use std::borrow::Cow;

/// For modeling comparison expression
#[derive(Debug, Clone, PartialEq)]
pub enum Compare<'a> {
    /// `left = right`
    Equals(Box<DatabaseValue<'a>>, Box<DatabaseValue<'a>>),
    /// `left <> right`
    NotEquals(Box<DatabaseValue<'a>>, Box<DatabaseValue<'a>>),
    /// `left < right`
    LessThan(Box<DatabaseValue<'a>>, Box<DatabaseValue<'a>>),
    /// `left <= right`
    LessThanOrEquals(Box<DatabaseValue<'a>>, Box<DatabaseValue<'a>>),
    /// `left > right`
    GreaterThan(Box<DatabaseValue<'a>>, Box<DatabaseValue<'a>>),
    /// `left >= right`
    GreaterThanOrEquals(Box<DatabaseValue<'a>>, Box<DatabaseValue<'a>>),
    /// `left IN (..)`
    In(Box<DatabaseValue<'a>>, Box<DatabaseValue<'a>>),
    /// `left NOT IN (..)`
    NotIn(Box<DatabaseValue<'a>>, Box<DatabaseValue<'a>>),
    /// `left LIKE %..%`
    Like(Box<DatabaseValue<'a>>, Cow<'a, str>),
    /// `left NOT LIKE %..%`
    NotLike(Box<DatabaseValue<'a>>, Cow<'a, str>),
    /// `left LIKE ..%`
    BeginsWith(Box<DatabaseValue<'a>>, Cow<'a, str>),
    /// `left NOT LIKE ..%`
    NotBeginsWith(Box<DatabaseValue<'a>>, Cow<'a, str>),
    /// `left LIKE %..`
    EndsInto(Box<DatabaseValue<'a>>, Cow<'a, str>),
    /// `left NOT LIKE %..`
    NotEndsInto(Box<DatabaseValue<'a>>, Cow<'a, str>),
    /// `value IS NULL`
    Null(Box<DatabaseValue<'a>>),
    /// `value IS NOT NULL`
    NotNull(Box<DatabaseValue<'a>>),
    /// `value` BETWEEN `left` AND `right`
    Between(
        Box<DatabaseValue<'a>>,
        Box<DatabaseValue<'a>>,
        Box<DatabaseValue<'a>>,
    ),
    /// `value` NOT BETWEEN `left` AND `right`
    NotBetween(
        Box<DatabaseValue<'a>>,
        Box<DatabaseValue<'a>>,
        Box<DatabaseValue<'a>>,
    ),
}

impl<'a> From<Compare<'a>> for ConditionTree<'a> {
    #[inline]
    fn from(cmp: Compare<'a>) -> Self {
        ConditionTree::single(Expression::from(cmp))
    }
}

impl<'a> From<Compare<'a>> for Expression<'a> {
    #[inline]
    fn from(cmp: Compare<'a>) -> Self {
        Expression::Compare(cmp)
    }
}

/// An item that can be compared against other values in the database.
pub trait Comparable<'a> {
    /// Tests if both sides are the same value.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".equals("bar"));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` = ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from("bar"),
    ///     ],
    ///     params
    /// );
    /// ```
    fn equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>;

    /// Tests if both sides are not the same value.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".not_equals("bar"));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` <> ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from("bar"),
    ///     ],
    ///     params
    /// );
    /// ```
    fn not_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>;

    /// Tests if the left side is smaller than the right side.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".less_than(10));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` < ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from(10),
    ///     ],
    ///     params
    /// );
    /// ```
    fn less_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>;

    /// Tests if the left side is smaller than the right side or the same.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".less_than_or_equals(10));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` <= ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from(10),
    ///     ],
    ///     params
    /// );
    /// ```
    fn less_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>;

    /// Tests if the left side is bigger than the right side.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".greater_than(10));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` > ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from(10),
    ///     ],
    ///     params
    /// );
    /// ```
    fn greater_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>;

    /// Tests if the left side is bigger than the right side or the same.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".greater_than_or_equals(10));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` >= ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from(10),
    ///     ],
    ///     params
    /// );
    /// ```
    fn greater_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>;

    /// Tests if the left side is included in the right side collection.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".in_selection(vec![1, 2]));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` IN (?, ?)", sql);
    /// assert_eq!(vec![
    ///     ParameterizedValue::Integer(1),
    ///     ParameterizedValue::Integer(2),
    /// ], params);
    /// ```
    fn in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>;

    /// Tests if the left side is not included in the right side collection.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".not_in_selection(vec![1, 2]));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` NOT IN (?, ?)", sql);
    ///
    /// assert_eq!(vec![
    ///     ParameterizedValue::Integer(1),
    ///     ParameterizedValue::Integer(2),
    /// ], params);
    /// ```
    fn not_in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>;

    /// Tests if the left side includes the right side string.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".like("bar"));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` LIKE ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from("%bar%"),
    ///     ],
    ///     params
    /// );
    /// ```
    fn like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>;

    /// Tests if the left side does not include the right side string.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".not_like("bar"));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` NOT LIKE ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from("%bar%"),
    ///     ],
    ///     params
    /// );
    /// ```
    fn not_like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>;

    /// Tests if the left side starts with the right side string.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".begins_with("bar"));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` LIKE ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from("bar%"),
    ///     ],
    ///     params
    /// );
    /// ```
    fn begins_with<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>;

    /// Tests if the left side doesn't start with the right side string.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".not_begins_with("bar"));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` NOT LIKE ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from("bar%"),
    ///     ],
    ///     params
    /// );
    /// ```
    fn not_begins_with<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>;

    /// Tests if the left side ends into the right side string.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".ends_into("bar"));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` LIKE ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from("%bar"),
    ///     ],
    ///     params
    /// );
    /// ```
    fn ends_into<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>;

    /// Tests if the left side does not end into the right side string.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".not_ends_into("bar"));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` NOT LIKE ?", sql);
    ///
    /// assert_eq!(
    ///     vec![
    ///         ParameterizedValue::from("%bar"),
    ///     ],
    ///     params
    /// );
    /// ```
    fn not_ends_into<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>;

    /// Tests if the left side is `NULL`.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".is_null());
    /// let (sql, _) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` IS NULL", sql);
    /// ```
    fn is_null(self) -> Compare<'a>;

    /// Tests if the left side is not `NULL`.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".is_not_null());
    /// let (sql, _) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` IS NOT NULL", sql);
    /// ```
    fn is_not_null(self) -> Compare<'a>;

    /// Tests if the value is between two given values.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".between(420, 666));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` BETWEEN ? AND ?", sql);
    ///
    /// assert_eq!(vec![
    ///     ParameterizedValue::Integer(420),
    ///     ParameterizedValue::Integer(666),
    /// ], params);
    /// ```
    fn between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
        V: Into<DatabaseValue<'a>>;

    /// Tests if the value is not between two given values.
    ///
    /// ```rust
    /// # use prisma_query::{ast::*, visitor::{Visitor, Sqlite}};
    /// let query = Select::from_table("users").so_that("foo".not_between(420, 666));
    /// let (sql, params) = Sqlite::build(query);
    ///
    /// assert_eq!("SELECT `users`.* FROM `users` WHERE `foo` NOT BETWEEN ? AND ?", sql);
    ///
    /// assert_eq!(vec![
    ///     ParameterizedValue::Integer(420),
    ///     ParameterizedValue::Integer(666),
    /// ], params);
    /// ```
    fn not_between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
        V: Into<DatabaseValue<'a>>;
}

impl<'a, U> Comparable<'a> for U
where
    U: Into<Column<'a>>,
{
    #[inline]
    fn equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();

        val.equals(comparison)
    }

    #[inline]
    fn not_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.not_equals(comparison)
    }

    #[inline]
    fn less_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.less_than(comparison)
    }

    #[inline]
    fn less_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.less_than_or_equals(comparison)
    }

    #[inline]
    fn greater_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.greater_than(comparison)
    }

    #[inline]
    fn greater_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.greater_than_or_equals(comparison)
    }

    #[inline]
    fn in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.in_selection(selection)
    }

    #[inline]
    fn not_in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.not_in_selection(selection)
    }

    #[inline]
    fn like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.like(pattern)
    }

    #[inline]
    fn not_like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.not_like(pattern)
    }

    #[inline]
    fn begins_with<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.begins_with(pattern)
    }

    #[inline]
    fn not_begins_with<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.not_begins_with(pattern)
    }

    #[inline]
    fn ends_into<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.ends_into(pattern)
    }

    #[inline]
    fn not_ends_into<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.not_ends_into(pattern)
    }

    #[inline]
    fn is_null(self) -> Compare<'a> {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.is_null()
    }

    #[inline]
    fn is_not_null(self) -> Compare<'a> {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.is_not_null()
    }

    #[inline]
    fn between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
        V: Into<DatabaseValue<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.between(left, right)
    }

    #[inline]
    fn not_between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<DatabaseValue<'a>>,
        V: Into<DatabaseValue<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: DatabaseValue<'a> = col.into();
        val.not_between(left, right)
    }
}
