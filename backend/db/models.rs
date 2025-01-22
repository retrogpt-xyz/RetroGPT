use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::tmsgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tmsg {
    pub id: i32,
    pub body: String,
    pub prnt_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::tmsgs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTmsg {
    pub body: String,
    pub prnt_id: Option<i32>,
}
