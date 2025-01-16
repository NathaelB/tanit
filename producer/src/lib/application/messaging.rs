use anyhow::Result;
use schema_registry_converter::{
    async_impl::schema_registry::{post_schema, SrSettings},
    schema_registry_common::{SchemaType, SuppliedSchema},
};

pub async fn create_ferri_schema() -> Result<()> {
    let schema_str = r#"
  {
      "type": "record",
      "name": "Ferry",
      "namespace": "nl.openweb.data",
      "fields": [
          {"name": "id", "type": "string"},
          {"name": "name", "type": "string"},
          {"name": "capacity", "type": "int"}
      ]
  }
  "#;

    let supplied_schema = SuppliedSchema {
        name: Some(String::from("nl.openweb.data.Ferry")),
        schema_type: SchemaType::Avro,
        schema: String::from(schema_str),
        references: vec![],
    };

    let sr_settings = SrSettings::new("http://localhost:8081".to_string());

    let registed_schema = post_schema(
        &sr_settings,
        "ferries-value".to_string(),
        supplied_schema.clone(),
    )
    .await?;

    println!("Schema registered with ID: {}", registed_schema.id);

    Ok(())
}

pub async fn create_car_schema() -> Result<()> {
    let schema_str = r#"
  {
      "type": "record",
      "name": "Car",
      "namespace": "nl.openweb.data",
      "fields": [
          {"name": "id", "type": "string"},
          {"name": "licence_plate", "type": "string"},
          {"name": "brand", "type": "string"},
          {"name": "color", "type": "string"},
          {"name": "capacity", "type": "int"}
      ]
  }
  "#;

    let supplied_schema = SuppliedSchema {
        name: Some(String::from("nl.openweb.data.Car")),
        schema_type: SchemaType::Avro,
        schema: String::from(schema_str),
        references: vec![],
    };

    let sr_settings = SrSettings::new("http://localhost:8081".to_string());

    let registed_schema = post_schema(
        &sr_settings,
        "cars-value".to_string(),
        supplied_schema.clone(),
    )
    .await?;

    println!("Schema registered with ID: {}", registed_schema.id);

    Ok(())
}

pub async fn create_passenger_schema() -> Result<()> {
    let schema_str = r#"
  {
      "type": "record",
      "name": "Passenger",
      "namespace": "nl.openweb.data",
      "fields": [
          {"name": "id", "type": "string"},
          {"name": "car_id", "type": ["null", "string"]},
          {"name": "ferry_id", "type": "string"},
          {"name": "firstname", "type": "string"},
          {"name": "lastname", "type": "string"},
          {"name": "sex", "type": "boolean"}
      ]
  }
  "#;

    let supplied_schema = SuppliedSchema {
        name: Some(String::from("nl.openweb.data.Passenger")),
        schema_type: SchemaType::Avro,
        schema: String::from(schema_str),
        references: vec![],
    };

    let sr_settings = SrSettings::new("http://localhost:8081".to_string());

    let registed_schema = post_schema(
        &sr_settings,
        "passengers-value".to_string(),
        supplied_schema.clone(),
    )
    .await?;

    println!("Schema registered with ID: {}", registed_schema.id);
    Ok(())
}
