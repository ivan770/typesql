use crate::{
    column::Column,
    from::From,
    query::{IsColumn, IsTable, Query, QueryWithoutTablesExt},
};

pub struct Select;

impl QueryWithoutTablesExt for Select {}

impl Query for Select {
    fn query() -> String {
        String::from("SELECT")
    }
}

pub trait SelectExt: Sized {
    fn column<Col: IsColumn>(self, column: Col) -> Column<Col, Self> {
        Column(column, self)
    }

    fn from<Table: IsTable>(self, table: Table) -> From<Table, Self> {
        From(table, self)
    }
}

impl SelectExt for Select {}

#[cfg(test)]
mod tests {
    use crate::query::QueryWithoutTablesExt;

    use super::Select;

    #[test]
    fn empty_select() {
        assert_eq!(Select.compile(), "SELECT");
    }
}
