require "json"
require "active_support/inflector"

require "./common"

spec = JSON.parse(File.read "searchfields.json")

lines = []

allentities = Hash.new {|hash, key| hash[key] = []}

spec.each do |field_name, field_props|
  field_type = field_props["type"]
  entities = field_props["entities"]

  lines << "/// #{field_props["comment"]}"
  lines << "pub struct #{field_name}(pub #{field_type});"

  lines << ""

  lines << "impl SearchField for #{field_name} {"
  lines << "    type Value = #{field_type};"
  lines << ""
  lines << "    fn name<R: Resource>(&self) -> Result<&'static str, Error> {"
  lines << "        match R::NAME {"

  field_props["entities"].each do |entity_name, field_value|
    lines<<"            full_entities::#{entity_name}::NAME => Ok(\"#{field_value}\"),"
    allentities[entity_name] << field_name
  end
  lines << "            s => Err(wrong_search_field(R::NAME, \"#{field_name}\")),"

  lines << "        }"
  lines << "    }"
  lines << ""
  lines << "    fn value(&self) -> String {"
  lines << "        format!(\"{}\", self.0)"
  lines << "    }"
  lines << ""
  lines << "    fn to_string<R: Resource>(&self) -> Result<String, Error> {"
  lines << "        Ok(format!(\"{}:{}\", self.name::<R>()?, self.value()))"
  lines << "    }"
  lines << "}"
  lines << ""
end

allentities.each do |entity, fields|
  lines << "/// Search fields for #{entity} entities."
  lines << "pub mod #{ActiveSupport::Inflector.underscore(entity)} {"
  fields.each do |field|
    lines << "    pub use super::#{field};"
  end
  lines << "}"
  lines << ""
end

code = lines.join("\n")

replace("src/search/fields.rs", "searchfields", code)