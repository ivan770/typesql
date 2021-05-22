pub trait Query {
    fn query() -> String;
}

pub trait IsTable {
    const NAME: &'static str;
}

pub trait IsColumn {
    type Table: IsTable;

    const NAME: &'static str;
}

pub trait TableProvider {
    type Tables;
}

pub trait TableRequirement {
    type Tables;
}

pub trait IgnoreTableRequirement {}

pub trait QueryExt: Query + Sized {
    fn compile(self) -> String {
        Self::query()
    }
}

pub trait QueryWithoutTablesExt: Query + Sized {
    fn compile(self) -> String {
        Self::query()
    }
}

impl<Q> QueryExt for Q where
    Q: TableProvider + TableRequirement<Tables = <Q as TableProvider>::Tables> + Query
{
}

impl<Q> QueryWithoutTablesExt for Q where Q: Query + IgnoreTableRequirement {}
