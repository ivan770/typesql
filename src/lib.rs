#![feature(specialization)]

mod column;
mod from;
mod query;
mod select;

pub fn select_query() -> select::Select {
    select::Select
}

#[cfg(test)]
mod tests {
    use crate::{
        column::ColumnExt,
        query::{IsColumn, IsTable, QueryExt},
        select::SelectExt,
        select_query,
    };

    struct Users;

    impl IsTable for Users {
        const NAME: &'static str = "users";
    }

    struct Posts;

    impl IsTable for Posts {
        const NAME: &'static str = "posts";
    }

    struct UsersId;

    impl IsColumn for UsersId {
        type Table = Users;

        const NAME: &'static str = "id";
    }

    struct UsersPostId;

    impl IsColumn for UsersPostId {
        type Table = Users;

        const NAME: &'static str = "post_id";
    }

    struct PostsId;

    impl IsColumn for PostsId {
        type Table = Posts;

        const NAME: &'static str = "id";
    }

    struct PostsTitle;

    impl IsColumn for PostsTitle {
        type Table = Posts;

        const NAME: &'static str = "title";
    }

    #[test]
    fn test_select_column() {
        let q = select_query().column(UsersId).from(Users).compile();

        assert_eq!(q, "SELECT users.id FROM users");
    }

    #[test]
    fn test_select_multiple_columns_from_one_table() {
        let q = select_query()
            .column(UsersId)
            // Looks like compiler is having a bad time infering an index here...
            .add_column::<frunk::indices::Here, _>(UsersPostId)
            .from(Users)
            .compile();

        assert_eq!(q, "SELECT users.id, users.post_id FROM users");
    }

    // FIXME: Doesn't compile even though it should.
    // #[test]
    // fn test_select_multiple_columns_from_multiple_tables() {
    //     let q = select_query()
    //         .column(UsersId)
    //         // Looks like compiler is having a bad time infering an index here...
    //         // FIXME: Correct index infering is required for safe column-from check
    //         .add_column::<frunk::indices::Here, _>(UsersPostId)
    //         .add_column::<frunk::indices::Here, _>(PostsId)
    //         .add_column::<frunk::indices::Here, _>(PostsTitle)
    //         .from(Users)
    //         .add_from::<frunk::indices::Here, _>(Posts)
    //         .compile();

    //     assert_eq!(q, "SELECT users.id, users.post_id, posts.id, posts.title FROM users, posts");
    // }
}
