use errors::*;
use diesel::prelude::*;
use json::LuaJsonValue;
use models::*;
use serde_json;


#[derive(Identifiable, Queryable, Serialize, PartialEq, Debug)]
#[table_name="emails"]
pub struct Email {
    pub id: i32,
    pub value: String,
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Model for Email {
    type ID = str;

    fn list(db: &Database) -> Result<Vec<Self>> {
        use schema::emails::dsl::*;

        let results = emails.load::<Self>(db.db())?;

        Ok(results)
    }

    fn filter(db: &Database, filter: &Filter) -> Result<Vec<Self>> {
        use schema::emails::dsl::*;

        let query = emails.filter(filter.sql());
        let results = query.load::<Self>(db.db())?;

        Ok(results)
    }

    fn by_id(db: &Database, my_id: i32) -> Result<Self> {
        use schema::emails::dsl::*;

        let domain = emails.filter(id.eq(my_id))
            .first::<Self>(db.db())?;

        Ok(domain)
    }

    fn id(db: &Database, query: &Self::ID) -> Result<i32> {
        use schema::emails::dsl::*;

        let domain_id = emails.filter(value.eq(query))
            .select(id)
            .first::<i32>(db.db())?;

        Ok(domain_id)
    }

    fn id_opt(db: &Database, query: &Self::ID) -> Result<Option<i32>> {
        use schema::emails::dsl::*;

        let domain_id = emails.filter(value.eq(query))
            .select(id)
            .first::<i32>(db.db())
            .optional()?;

        Ok(domain_id)
    }
}

pub struct PrintableEmail {
    value: String,
}

impl fmt::Display for PrintableEmail {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{:?}", self.value)
    }
}

impl Printable<PrintableEmail> for Email {
    fn printable(&self, _db: &Database) -> Result<PrintableEmail> {
        Ok(PrintableEmail {
            value: self.value.to_string(),
        })
    }
}

pub struct DetailedEmail {
    id: i32,
    value: String,
}

impl fmt::Display for DetailedEmail {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "\x1b[32m#{}\x1b[0m, \x1b[32m{:?}\x1b[0m", self.id, self.value)
    }
}

impl Detailed for Email {
    type T = DetailedEmail;

    fn detailed(&self, _db: &Database) -> Result<Self::T> {
        Ok(DetailedEmail {
            id: self.id,
            value: self.value.to_string(),
        })
    }
}

#[derive(Insertable)]
#[table_name="emails"]
pub struct NewEmail<'a> {
    pub value: &'a str,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="emails"]
pub struct NewEmailOwned {
    pub value: String,
}

impl NewEmailOwned {
    pub fn from_lua(x: LuaJsonValue) -> Result<NewEmailOwned> {
        let x = serde_json::from_value(x.into())?;
        Ok(x)
    }
}

impl Printable<PrintableEmail> for NewEmailOwned {
    fn printable(&self, _db: &Database) -> Result<PrintableEmail> {
        Ok(PrintableEmail {
            value: self.value.to_string(),
        })
    }
}