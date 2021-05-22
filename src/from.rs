use std::marker::PhantomData;

use frunk::{hlist::Selector, HCons, HNil};

use crate::query::{IsTable, Query, TableProvider, TableRequirement};

pub struct From<Table, Inner>(pub(crate) Table, pub(crate) Inner);

impl<Table, Inner> TableRequirement for From<Table, Inner>
where
    Inner: TableRequirement,
{
    type Tables = Inner::Tables;
}

impl<Table, Inner> TableProvider for From<Table, Inner>
where
    Table: IsTable,
{
    type Tables = HCons<Table, HNil>;
}

impl<Table, Inner> Query for From<Table, Inner>
where
    Table: IsTable,
    Inner: Query,
{
    fn query() -> String {
        format!("{} FROM {}", Inner::query(), Table::NAME)
    }
}

pub struct AddFrom<Table, Inner, HListIndex>(Table, Inner, PhantomData<HListIndex>);

impl<Table, Inner, HListIndex> TableRequirement for AddFrom<Table, Inner, HListIndex>
where
    Inner: TableRequirement,
{
    type Tables = Inner::Tables;
}

impl<Table, Inner, HListIndex> TableProvider for AddFrom<Table, Inner, HListIndex>
where
    Table: IsTable,
    Inner: TableProvider,
{
    default type Tables = HCons<Table, Inner::Tables>;
}

impl<Table, Inner, HListIndex> TableProvider for AddFrom<Table, Inner, HListIndex>
where
    Table: IsTable,
    Inner: TableProvider,
    <Inner as TableProvider>::Tables: Selector<Table, HListIndex>,
{
    type Tables = Inner::Tables;
}

impl<Table, HListIndex, InnerFromQuery, InnerFromTable> Query
    for AddFrom<Table, From<InnerFromTable, InnerFromQuery>, HListIndex>
where
    InnerFromQuery: Query,
    InnerFromTable: IsTable,
    Table: IsTable,
{
    fn query() -> String {
        format!(
            "{}, {}",
            From::<InnerFromTable, InnerFromQuery>::query(),
            Table::NAME
        )
    }
}

impl<Table, Inner, HListIndex> Query for AddFrom<Table, Inner, HListIndex>
where
    Inner: Query,
    Table: IsTable,
{
    default fn query() -> String {
        format!("{}, {}", Inner::query(), Table::NAME)
    }
}

pub trait FromExt: Sized {
    fn add_from<HListIndex, AddTable: IsTable>(
        self,
        table: AddTable,
    ) -> AddFrom<AddTable, Self, HListIndex> {
        AddFrom(table, self, PhantomData::default())
    }
}

impl<Table, Inner> FromExt for From<Table, Inner>
where
    Table: IsTable,
    Inner: Query,
{
}

impl<Table, Inner, HListIndex> FromExt for AddFrom<Table, Inner, HListIndex>
where
    Table: IsTable,
    Inner: Query,
{
}
