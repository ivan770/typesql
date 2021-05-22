use std::marker::PhantomData;

use frunk::{hlist::Selector, HCons, HNil};

use crate::{
    from::From,
    query::{IsColumn, IsTable, Query, TableRequirement},
};

pub struct Column<Col, Inner>(pub(crate) Col, pub(crate) Inner);

impl<Col, Inner> TableRequirement for Column<Col, Inner>
where
    Col: IsColumn,
{
    type Tables = HCons<Col::Table, HNil>;
}

impl<Col, Inner> Query for Column<Col, Inner>
where
    Col: IsColumn,
    Inner: Query,
{
    fn query() -> String {
        format!(
            "{} {}.{}",
            Inner::query(),
            <Col::Table as IsTable>::NAME,
            Col::NAME
        )
    }
}

pub struct AddColumn<Col, Inner, HListIndex>(Col, Inner, PhantomData<HListIndex>);

impl<Col, Inner, HListIndex> TableRequirement for AddColumn<Col, Inner, HListIndex>
where
    Col: IsColumn,
    Inner: TableRequirement,
{
    default type Tables = HCons<Col::Table, Inner::Tables>;
}

impl<Col, Inner, HListIndex> TableRequirement for AddColumn<Col, Inner, HListIndex>
where
    Col: IsColumn,
    Inner: TableRequirement,
    <Inner as TableRequirement>::Tables: Selector<Col::Table, HListIndex>,
{
    type Tables = Inner::Tables;
}

impl<Col, Inner, HListIndex> Query for AddColumn<Col, Inner, HListIndex>
where
    Col: IsColumn,
    Inner: Query,
{
    default fn query() -> String {
        format!(
            "{}, {}.{}",
            Inner::query(),
            <Col::Table as IsTable>::NAME,
            Col::NAME
        )
    }
}

pub trait ColumnExt: Sized {
    fn add_column<HListIndex, AddCol: IsColumn>(
        self,
        column: AddCol,
    ) -> AddColumn<AddCol, Self, HListIndex> {
        AddColumn(column, self, PhantomData::default())
    }

    fn from<Table: IsTable>(self, table: Table) -> From<Table, Self> {
        From(table, self)
    }
}

impl<Col, Inner> ColumnExt for Column<Col, Inner>
where
    Col: IsColumn,
    Inner: Query,
{
}

impl<Col, Inner, HListIndex> ColumnExt for AddColumn<Col, Inner, HListIndex>
where
    Col: IsColumn,
    Inner: Query,
{
}
