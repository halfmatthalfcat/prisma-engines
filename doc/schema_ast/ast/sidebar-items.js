initSidebarItems({"enum":[["AttributeContainer","A node containing attributes."],["Expression","Represents arbitrary, even nested, expressions."],["FieldArity",""],["FieldPosition","In a scalar field."],["FieldType",""],["ModelPosition","A cursor position in a context."],["SchemaPosition","A cursor position in a schema."],["Top","Enum for distinguishing between top-level entries"],["TopId","An identifier for a top-level item in a schema AST. Use the `schema[top_id]` syntax to resolve the id to an `ast::Top`."]],"struct":[["AliasId","An opaque identifier for a type alias in a schema AST. Use the `schema[alias_id]` syntax to resolve the id to an `ast::Field`."],["Argument","An argument, either for attributes or for function call expressions."],["ArgumentsList","A list of arguments inside parentheses."],["Attribute","An attribute (following `@` or `@@``) on a model, model field, enum, enum value or composite type field."],["AttributeId","An attribute (@ or @@) node in the AST."],["Comment","A comment. Currently unimplemented."],["CompositeType","A composite type declaration."],["CompositeTypeId","An opaque identifier for a type definition in a schema AST. Use the `schema[type_id]` syntax to resolve the id to an `ast::CompositeType`."],["ConfigBlockProperty","A named property in a config block."],["EmptyArgument","An argument with a name but no value. Example:"],["Enum","An enum declaration."],["EnumId","An opaque identifier for an enum in a schema AST."],["EnumValue","An enum value definition."],["Field",""],["FieldId","An opaque identifier for a field in an AST model. Use the `model[field_id]` syntax to resolve the id to an `ast::Field`."],["GeneratorConfig","A Generator block declaration."],["GeneratorId","An opaque identifier for a generator block in a schema AST."],["Identifier","An identifier."],["Model","A model declaration."],["ModelId","An opaque identifier for a model in a schema AST. Use the `schema[model_id]` syntax to resolve the id to an `ast::Model`."],["SchemaAst","AST representation of a prisma schema."],["SourceConfig","A source block declaration."],["SourceId","An opaque identifier for a datasource blèck in a schema AST."],["Span","Represents a location in a datamodel’s text representation."]],"trait":[["WithAttributes","An AST node with attributes."],["WithDocumentation","An AST node with documentation."],["WithIdentifier","An AST node with an identifier."],["WithName","An AST node with a name (from the identifier)."],["WithSpan","An AST node with a span."]]});