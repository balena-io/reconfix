
use std::borrow::Cow;
use std::cmp::Ordering;
use std::str::FromStr;

use error::*;
use super::types::*;
use schema::types::{self as schema, Schema, ObjectSchema, Map, TypeKind};
use json::Pointer as JsonPointer;

use uuid::Uuid;

trait Generator {
    fn generate(&self, schema: &Schema) -> Result<Vec<Transform>>;
}

pub struct DefaultGenerator;

impl Generator for DefaultGenerator {
    fn generate(&self, schema: &Schema) -> Result<Vec<Transform>> {
        let root = match *schema {
            Schema::Boolean(_) => return Ok(Vec::new()),
            Schema::Object(ref obj) => obj,
        };

        let root_ctx = Context::new();

        get_transforms(&root, &root_ctx)
    }
}

#[derive(Clone)]
struct Context {
    selector: Selector,
    targets: Map<String, Target>,
}

impl Context {
    fn new() -> Context {
        Context {
            selector: Selector::new(),
            targets: Map::new(),
        }
    }

    fn add_component(&self, component: Component) -> Context {
        let mut ctx = self.clone();
        ctx.selector.push(component);
        ctx
    }

    fn add_targets(&self, targets: &Map<String, Target>) -> Context {
        let mut ctx = self.clone();
        for (key, target) in targets {
            ctx.targets.insert(key.clone(), target.clone());
        }
        ctx
    }

    fn get_target(&self, name: &str) -> Result<&Target> {
        match self.targets.get(name) {
            Some(t) => Ok(t),
            None => bail!("unable to find target '{}'", name),
        }
    }
}

fn get_transforms(obj: &ObjectSchema, ctx: &Context) -> Result<Vec<Transform>> {
    let mut transforms = Vec::new();
    let mut ctx = Cow::Borrowed(ctx);

    if let Some(ref reconfix) = obj.reconfix {
        let (exp_ctx, current) = process_reconfix_object(reconfix, ctx.as_ref())?;
        transforms.extend(current);
        ctx = Cow::Owned(exp_ctx);
    }

    for (name, schema) in obj.properties.iter() {
        let prop_obj = match *schema {
            Schema::Object(ref o) => o,
            Schema::Boolean(_) => continue,
        };

        let prop_ctx = ctx.add_component(Component::Property(Key::Single(name.clone())));
        let prop_transforms = get_transforms(&prop_obj, &prop_ctx)?;
        transforms.extend(prop_transforms);
    }

    match obj.items {
        Some(TypeKind::Single(Schema::Object(ref obj))) => {
            let item_ctx = ctx.add_component(Component::Item(Index::Wildcard));
            let item_transforms = get_transforms(obj.as_ref(), &item_ctx)?;      
            transforms.extend(item_transforms); 
        },
        Some(TypeKind::Set(ref schemas)) => {
            for (index, schema) in schemas.iter().enumerate() {
                if let Schema::Object(ref obj) = *schema {
                    let item_ctx = ctx.add_component(Component::Item(Index::Single(index as u64)));
                    let item_transforms = get_transforms(obj.as_ref(), &item_ctx)?;      
                    transforms.extend(item_transforms); 
                }
            }
        }, 
        _ => (),
    }

    if let Some(Schema::Object(ref obj)) = obj.additional_items {
        let item_ctx = ctx.add_component(Component::Item(Index::Wildcard));
        let item_transforms = get_transforms(obj.as_ref(), &item_ctx)?;      
        transforms.extend(item_transforms);
    }

    Ok(transforms)
}

fn process_reconfix_object(reconfix: &schema::Reconfix, ctx: &Context) -> Result<(Context, Vec<Transform>)> {
    let converted_targets = convert_targets(&reconfix.targets, ctx)?;
    let context = ctx.add_targets(&converted_targets);

    let transforms = reconfix.transforms
        .iter()
        .map(|t| convert_transform(t, &context))
        .collect::<Result<Vec<_>>>()?;
    
    Ok((context, transforms))
}

fn convert_transform(transform: &schema::Transform, ctx: &Context) -> Result<Transform> {
    let target = ctx.get_target(transform.output.target.as_ref())?;
    let map = match transform.map {
        Some(ref t) => match *t {
            schema::TypeKind::Single(ref case) => vec![convert_case(&case)],
            schema::TypeKind::Set(ref cases) => cases.iter().map(convert_case).collect(),
        },
        None => vec![Case::Identity],
    };

    let destination = JsonPointer::from_str(transform.output.path.as_ref())
        .chain_err(|| "unable to parse JSON pointer")?;

    Ok(Transform {
        source: ctx.selector.clone(),
        target: target.clone(),
        destination: destination,
        map: map,
    })
}

fn convert_case(case: &schema::Case) -> Case {
    match *case {
        schema::Case::Identity => Case::Identity,
        schema::Case::Tuple(ref val, ref schema) => match schema {
            &Schema::Object(ref obj) => match obj.as_ref() {
                &ObjectSchema { const_: Some(ref c), .. } => {
                    Case::Value { dry: val.clone(), wet: c.clone() }
                },
                o => Case::Template { dry: val.clone(), template: Schema::Object(Box::new(o.clone())) },
            },
            s => Case::Template { dry: val.clone(), template: s.clone() },
        },
    }
}

fn convert_targets(targets: &Map<String, schema::Target>, ctx: &Context) -> Result<Map<String, Target>> {
    let mut local_targets = Map::new();

    let mut sorted = targets.iter().collect::<Vec<_>>();
    sorted.sort_unstable_by(|x, y| {
        match (x.1, y.1) {
            (&schema::Target::File { location: schema::Location::Disk { .. }, .. }, _) => Ordering::Less,
            (_, &schema::Target::File { location: schema::Location::Disk { .. }, .. }) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    });

    for (name, target) in sorted {
        let target = match *target {
            schema::Target::File { ref format, ref location } => {
                let format = match *format {
                    schema::Format::Json => Format::Json,
                    schema::Format::Ini => Format::Ini,
                };

                let location = match *location {
                    schema::Location::Disk { ref partition, ref path } => {
                        Location::Disk(DiskFile {
                            partition: convert_partition(partition),
                            path: path.to_string(),
                        })
                    },
                    schema::Location::Nested { ref file, ref path } => {
                        Location::Nested(NestedFile {
                            file: dereference_disk(file, Some(&local_targets), ctx)?,
                            path: JsonPointer::from_str(path)
                                .chain_err(|| "unable to parse path pointer")?,
                        })
                    }
                };

                Target::File(File {
                    format: format,
                    location: location,
                })
            },
            schema::Target::NetworkManager => Target::NetworkManager,
        };

        local_targets.insert(name.to_string(), target);
    }

    Ok(local_targets)
}

fn dereference_disk(target: &str, local: Option<&Map<String, Target>>, ctx: &Context) -> Result<DiskFile> {
    for opt in [local, Some(&ctx.targets)].iter() {
        let map = match *opt {
            Some(ref m) => m,
            None => continue,
        };

        match map.get(target) {
            Some(&Target::File(File { location: Location::Disk(ref d), .. })) => {
                return Ok(d.clone());
            },
            Some(_) => bail!("'{}' is not a valid nesting target", target),
            _ => (),
        }
    }

    bail!("target '{}' not found", target);
}

fn convert_partition(partition: &schema::Partition) -> Partition {
    match *partition {
        schema::Partition::String(ref s) => {
            match Uuid::parse_str(s) {
                Ok(uuid) => Partition::Id(uuid),
                _ => Partition::Label(s.to_string()),
            }
        },
        schema::Partition::Number(ref idx) => Partition::Number(*idx),
    }
}
