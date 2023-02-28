
import jsonSchema from '../model/json-schema.json'

export function getNoAdditionalPropertiesSchema<T> (
  modelName: string,
  model: T
): T {
  const modelJsonSchema: {
    properties: any[]
  } | undefined = jsonSchema?.definitions?.[modelName] ?? undefined
  if (modelJsonSchema == null) {
    throw new Error(`Failed to find json schema for model: ${modelName}`)
  }

  const properties = modelJsonSchema.properties as Record<string, any>

  // FIXME: make this work with typescript
  return Object.fromEntries(
    Object.entries(model as Record<string, any>).filter(([key, value]) => {
      if (value === null) return false

      return properties[key] != null
    })
  ) as T
}
