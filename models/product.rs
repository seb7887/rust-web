use crate::schema::products;

#[derive(Queryable)]
pub struct Product {
  pub id: i32,
  pub name: String,
  pub stock: f64,
  pub price: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "products"]
pub struct NewProduct {
  pub name: Option<String>,
  pub stock: Option<f64>,
  pub price: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct ProductList(pub Vec<Product>);

impl ProductList {
  pub fn list() -> Self {
    use crate::db::establish_connection;
    use crate::schema::products::dsl::*;
    use diesel::{QueryDsl, RunQueryDsl};

    let connection = establish_connection();

    let result = products
      .limit(10)
      .load::<Product>(&connection)
      .expect("Error loading products");

    ProductList(result)
  }
}
