//! Determining which types has typed parameters in array.

use super::{ConstrainResult, MonotoneFramework, generate_dependencies};
use std::collections::HashSet;
use std::collections::HashMap;
use ir::context::{BindgenContext, ItemId};
use ir::traversal::EdgeKind;
use ir::ty::TypeKind;
use ir::comp::Field;
use ir::comp::FieldMethods;

/// An analysis that finds for each IR item whether it has array or not.
///
/// We use the monotone constraint function `has_type_parameter_in_array`,
/// defined as follows:
///
/// * If T is Array type with type parameter, T trivially has.
/// * If T is a type alias, a templated alias or an indirection to another type,
///   it has type parameter in array if the type T refers to has.
/// * If T is a compound type, it has array if any of base memter or field
///   has type paramter in array.
/// * If T is an instantiation of an abstract template definition, T has
///   type parameter in array if any of the template arguments or template definition
///   has.
#[derive(Debug, Clone)]
pub struct HasTypeParameterInArray<'ctx, 'gen>
    where 'gen: 'ctx
{
    ctx: &'ctx BindgenContext<'gen>,

    // The incremental result of this analysis's computation. Everything in this
    // set has array.
    has_type_parameter_in_array: HashSet<ItemId>,

    // Dependencies saying that if a key ItemId has been inserted into the
    // `has_type_parameter_in_array` set, then each of the ids in Vec<ItemId> need to be
    // considered again.
    //
    // This is a subset of the natural IR graph with reversed edges, where we
    // only include the edges from the IR graph that can affect whether a type
    // has array or not.
    dependencies: HashMap<ItemId, Vec<ItemId>>,
}

impl<'ctx, 'gen> HasTypeParameterInArray<'ctx, 'gen> {
    fn consider_edge(kind: EdgeKind) -> bool {
        match kind {
            // These are the only edges that can affect whether a type can derive
            // debug or not.
            EdgeKind::BaseMember |
            EdgeKind::Field |
            EdgeKind::TypeReference |
            EdgeKind::VarType |
            EdgeKind::TemplateArgument |
            EdgeKind::TemplateDeclaration |
            EdgeKind::TemplateParameterDefinition => true,

            EdgeKind::Constructor |
            EdgeKind::Destructor |
            EdgeKind::FunctionReturn |
            EdgeKind::FunctionParameter |
            EdgeKind::InnerType |
            EdgeKind::InnerVar |
            EdgeKind::Method => false,
            EdgeKind::Generic => false,
        }
    }

    fn insert(&mut self, id: ItemId) -> ConstrainResult {
        trace!("inserting {:?} into the has_type_parameter_in_array set", id);

        let was_not_already_in_set = self.has_type_parameter_in_array.insert(id);
        assert!(
            was_not_already_in_set,
            "We shouldn't try and insert {:?} twice because if it was \
             already in the set, `constrain` should have exited early.",
            id
        );

        ConstrainResult::Changed
    }
}

impl<'ctx, 'gen> MonotoneFramework for HasTypeParameterInArray<'ctx, 'gen> {
    type Node = ItemId;
    type Extra = &'ctx BindgenContext<'gen>;
    type Output = HashSet<ItemId>;

    fn new(ctx: &'ctx BindgenContext<'gen>) -> HasTypeParameterInArray<'ctx, 'gen> {
        let has_type_parameter_in_array = HashSet::new();
        let dependencies = generate_dependencies(ctx, Self::consider_edge);

        HasTypeParameterInArray {
            ctx,
            has_type_parameter_in_array,
            dependencies,
        }
    }

    fn initial_worklist(&self) -> Vec<ItemId> {
        self.ctx.whitelisted_items().iter().cloned().collect()
    }

    fn constrain(&mut self, id: ItemId) -> ConstrainResult {
        trace!("constrain: {:?}", id);

        if self.has_type_parameter_in_array.contains(&id) {
            trace!("    already know it do not have array");
            return ConstrainResult::Same;
        }

        let item = self.ctx.resolve_item(id);
        let ty = match item.as_type() {
            Some(ty) => ty,
            None => {
                trace!("    not a type; ignoring");
                return ConstrainResult::Same;
            }
        };

        match *ty.kind() {
            // Handle the simple cases. These can derive copy without further
            // information.
            TypeKind::Void |
            TypeKind::NullPtr |
            TypeKind::Int(..) |
            TypeKind::Float(..) |
            TypeKind::Complex(..) |
            TypeKind::Function(..) |
            TypeKind::Enum(..) |
            TypeKind::Reference(..) |
            TypeKind::BlockPointer |
            TypeKind::Named |
            TypeKind::Opaque |
            TypeKind::Pointer(..) |
            TypeKind::UnresolvedTypeRef(..) |
            TypeKind::ObjCInterface(..) |
            TypeKind::ObjCId |
            TypeKind::ObjCSel => {
                trace!("    simple type that do not have array");
                ConstrainResult::Same
            }

            TypeKind::Array(t, _) => {
                let inner_ty = self.ctx.resolve_type(t).canonical_type(self.ctx);
                match *inner_ty.kind() {
                    TypeKind::Named => {
                        trace!("    Array with Named type has type parameter");
                        self.insert(id)
                    }
                    _ => {
                        trace!("    Array without Named type does have type parameter");
                        ConstrainResult::Same
                    }
                }
            }

            TypeKind::ResolvedTypeRef(t) |
            TypeKind::TemplateAlias(t, _) |
            TypeKind::Alias(t) => {
                if self.has_type_parameter_in_array.contains(&t) {
                    trace!("    aliases and type refs to T which have array \
                            also have array");
                    self.insert(id)
                } else {
                    trace!("    aliases and type refs to T which do not have array \
                            also do not have array");
                    ConstrainResult::Same
                }
            }

            TypeKind::Comp(ref info) => {
                let bases_have = info.base_members()
                    .iter()
                    .any(|base| self.has_type_parameter_in_array.contains(&base.ty));
                if bases_have {
                    trace!("    bases have array, so we also have");
                    return self.insert(id);
                }
                let fields_have = info.fields()
                    .iter()
                    .any(|f| {
                        match *f {
                            Field::DataMember(ref data) => {
                                self.has_type_parameter_in_array.contains(&data.ty())
                            }
                            Field::Bitfields(..) => false,
                        }
                    });
                if fields_have {
                    trace!("    fields have array, so we also have");
                    return self.insert(id);
                }

                trace!("    comp doesn't have array");
                ConstrainResult::Same
            }

            TypeKind::TemplateInstantiation(ref template) => {
                let args_have = template.template_arguments()
                    .iter()
                    .any(|arg| self.has_type_parameter_in_array.contains(&arg));
                if args_have {
                    trace!("    template args have array, so \
                            insantiation also has array");
                    return self.insert(id);
                }

                let def_has = self.has_type_parameter_in_array
                    .contains(&template.template_definition());
                if def_has {
                    trace!("    template definition has array, so \
                            insantiation also has");
                    return self.insert(id);
                }

                trace!("    template instantiation do not have array");
                ConstrainResult::Same
            }
        }
    }

    fn each_depending_on<F>(&self, id: ItemId, mut f: F)
        where F: FnMut(ItemId),
    {
        if let Some(edges) = self.dependencies.get(&id) {
            for item in edges {
                trace!("enqueue {:?} into worklist", item);
                f(*item);
            }
        }
    }
}

impl<'ctx, 'gen> From<HasTypeParameterInArray<'ctx, 'gen>> for HashSet<ItemId> {
    fn from(analysis: HasTypeParameterInArray<'ctx, 'gen>) -> Self {
        analysis.has_type_parameter_in_array
    }
}
