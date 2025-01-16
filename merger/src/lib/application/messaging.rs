use anyhow::Result;
use schema_registry_converter::{
    async_impl::schema_registry::{post_schema, SrSettings},
    schema_registry_common::{SchemaType, SuppliedSchema},
};
use tracing::info;

pub async fn create_saver_schema() -> Result<()> {
    let schema_str = r#"
  {
      "type": "record",
      "name": "Ferry",
      "fields": [
          {"name": "id", "type": "string"},
          {"name": "name", "type": "string"},
          {"name": "capacity", "type": "int"},
          {"name": "passengers", "type": {
              "type": "array",
              "items": {
                  "type": "record",
                  "name": "Passenger",
                  "fields": [
                      {"name": "id", "type": "string"},
                      {"name": "car_id", "type": ["null", "string"]},
                      {"name": "ferry_id", "type": "string"},
                      {"name": "firstname", "type": "string"},
                      {"name": "lastname", "type": "string"},
                      {"name": "sex", "type": "boolean"}
                  ]
              }
          }},
          {"name": "cars", "type": {
              "type": "array",
              "items": {
                  "type": "record",
                  "name": "Car",
                  "fields": [
                      {"name": "id", "type": "string"},
                      {"name": "brand", "type": "string"},
                      {"name": "color", "type": "string"},
                      {"name": "capacity", "type": "int"}
                  ]
              }
          }}
      ]
  }
  "#;

    let supplied_schema = SuppliedSchema {
        name: Some(String::from("nl.openweb.data.Saver")),
        schema_type: SchemaType::Avro,
        schema: String::from(schema_str),
        references: vec![],
    };

    let sr_settings = SrSettings::new("http://localhost:8081".to_string());

    let registed_schema = post_schema(
        &sr_settings,
        "saver-value".to_string(),
        supplied_schema.clone(),
    )
    .await?;

    info!("Schema registered with ID: {}", registed_schema.id);
    Ok(())
}
