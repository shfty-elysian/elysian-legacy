use elysian_core::ir::{
    ast::{
        Expr, Number, Stmt, Struct, Value, COLOR, MATRIX2, MATRIX3, MATRIX4, POSITION_2D, VECTOR2,
        VECTOR3, VECTOR4,
    },
    module::{
        FunctionDefinition, Module as ElysianModule, NumericType, PropertyIdentifier,
        Type as ElysianType, CONTEXT, SAFE_NORMALIZE_2, SAFE_NORMALIZE_3, SAFE_NORMALIZE_4,
    },
};
use elysian_decl_macros::elysian_function;
use elysian_shapes::modify::ASPECT;
use indexmap::IndexMap;
use naga::{
    valid::{Capabilities, ModuleInfo, ValidationError, ValidationFlags, Validator},
    Arena, BinaryOperator, Block as NagaBlock, EntryPoint, Expression, Function, FunctionArgument,
    FunctionResult, Handle, Literal, LocalVariable, MathFunction, Module as NagaModule, Range,
    ScalarKind, ShaderStage, Span, Statement, StructMember, SwizzleComponent, Type as NagaType,
    TypeInner, UniqueArena, VectorSize, WithSpan,
};

#[derive(Debug, Default)]
pub struct ExpressionQueue {
    expressions: Arena<Expression>,
    queue: Vec<Handle<Expression>>,
}

#[derive(Debug, Default)]
pub struct LocalVariableStore {
    locals: Arena<LocalVariable>,
    pointers: IndexMap<Handle<LocalVariable>, Handle<Expression>>,
}

pub struct NagaBuilder<'a> {
    input: &'a ElysianModule,
    types: UniqueArena<NagaType>,
    functions: Arena<Function>,
    function: Option<FunctionDefinition>,
    block_stack: Vec<NagaBlock>,
    expressions: Option<ExpressionQueue>,
    local_variables: Option<LocalVariableStore>,
}

impl<'a> NagaBuilder<'a> {
    pub fn new(module: &'a ElysianModule) -> Self {
        NagaBuilder {
            input: module,
            types: Default::default(),
            functions: Default::default(),
            function: Default::default(),
            block_stack: Default::default(),
            expressions: Default::default(),
            local_variables: Default::default(),
        }
    }

    pub fn build(
        mut self,
        validation_flags: ValidationFlags,
        capabilities: Capabilities,
    ) -> Result<(NagaModule, ModuleInfo), WithSpan<ValidationError>> {
        #[cfg(feature = "print")]
        println!("module_to_naga");

        self.types_to_naga();
        self.functions_to_naga();
        let entry_point = self.shadertoy_entry_point();

        let module = NagaModule {
            types: self.types,
            special_types: Default::default(),
            constants: Default::default(),
            global_variables: Default::default(),
            const_expressions: Default::default(),
            functions: self.functions,
            entry_points: vec![entry_point],
        };

        let mut validator = Validator::new(validation_flags, capabilities);
        let module_info = validator.validate(&module)?;

        Ok((module, module_info))
    }

    fn get_type(&self, name: &str) -> (Handle<NagaType>, &NagaType) {
        #[cfg(feature = "print")]
        println!("get_type");

        self.types
            .iter()
            .find(|(_, v)| v.name.as_ref().map(String::as_str) == Some(name))
            .unwrap_or_else(|| panic!("No type for {}", name))
    }

    fn get_function(&self, name: &str) -> (Handle<Function>, &Function) {
        #[cfg(feature = "print")]
        println!("get_function");

        self.functions
            .iter()
            .find(|(_, v)| v.name.as_ref().map(String::as_str) == Some(name))
            .unwrap_or_else(|| panic!("No function for {name:}"))
    }

    fn get_local_variable(&self, name: &str) -> Option<(Handle<LocalVariable>, &LocalVariable)> {
        #[cfg(feature = "print")]
        println!("get_local_variable");

        self.local_variables
            .as_ref()?
            .locals
            .iter()
            .find(|(_, v)| v.name.as_ref().map(String::as_str) == Some(name))
    }

    fn get_pointer(&self, name: &str) -> Option<&Handle<Expression>> {
        #[cfg(feature = "print")]
        println!("get_pointer");

        let (k, _) = self.get_local_variable(name)?;
        self.local_variables
            .as_ref()
            .expect("Not in a block")
            .pointers
            .get(&k)
    }

    fn types_to_naga(&mut self) {
        #[cfg(feature = "print")]
        println!("types_to_naga");

        let bool = self.push_type(NagaType {
            name: Some("Bool".to_string()),
            inner: TypeInner::Scalar {
                kind: ScalarKind::Bool,
                width: 1,
            },
        });

        let uint = self.push_type(NagaType {
            name: Some("UInt".to_string()),
            inner: TypeInner::Scalar {
                kind: ScalarKind::Uint,
                width: 4,
            },
        });

        let sint = self.push_type(NagaType {
            name: Some("SInt".to_string()),
            inner: TypeInner::Scalar {
                kind: ScalarKind::Sint,
                width: 4,
            },
        });

        let float = self.push_type(NagaType {
            name: Some("Float".to_string()),
            inner: TypeInner::Scalar {
                kind: ScalarKind::Float,
                width: 4,
            },
        });

        for def in &self.input.struct_definitions {
            let ty = match &def.id {
                v if **v == VECTOR2 || **v == VECTOR3 || **v == VECTOR4 => NagaType {
                    name: Some(def.id.name().to_string()),
                    inner: TypeInner::Vector {
                        size: match &def.id {
                            d if **d == VECTOR2 => VectorSize::Bi,
                            d if **d == VECTOR3 => VectorSize::Tri,
                            d if **d == VECTOR4 => VectorSize::Quad,
                            _ => unreachable!(),
                        },
                        kind: ScalarKind::Float,
                        width: 4,
                    },
                },
                m if **m == MATRIX2 || **m == MATRIX3 || **m == MATRIX4 => NagaType {
                    name: Some(def.id.name().to_string()),
                    inner: TypeInner::Matrix {
                        columns: match &def.id {
                            d if **d == MATRIX2 => VectorSize::Bi,
                            d if **d == MATRIX3 => VectorSize::Tri,
                            d if **d == MATRIX4 => VectorSize::Quad,
                            _ => unreachable!(),
                        },
                        rows: match &def.id {
                            d if **d == MATRIX2 => VectorSize::Bi,
                            d if **d == MATRIX3 => VectorSize::Tri,
                            d if **d == MATRIX4 => VectorSize::Quad,
                            _ => unreachable!(),
                        },
                        width: 4,
                    },
                },
                _ => {
                    let (members, span) =
                        def.fields
                            .iter()
                            .fold((vec![], 0), |(mut members, total_span), next| {
                                let (member, span) = match self.get_input_type(&next.id) {
                                    ElysianType::Boolean => (bool, 1),
                                    ElysianType::Number(n) => match n {
                                        NumericType::UInt => (uint, 4),
                                        NumericType::SInt => (sint, 4),
                                        NumericType::Float => (float, 4),
                                    },
                                    ElysianType::Struct(s) => {
                                        let (handle, ty) = self.get_type(s.name());

                                        let span = match ty.inner {
                                            TypeInner::Scalar { width, .. } => width as u32,
                                            TypeInner::Vector { width, size, .. } => {
                                                width as u32
                                                    * match size {
                                                        VectorSize::Bi => 2,
                                                        VectorSize::Tri => 3,
                                                        VectorSize::Quad => 4,
                                                    }
                                            }
                                            TypeInner::Matrix {
                                                columns,
                                                rows,
                                                width,
                                            } => {
                                                width as u32
                                                    * match columns {
                                                        VectorSize::Bi => 2,
                                                        VectorSize::Tri => 3,
                                                        VectorSize::Quad => 4,
                                                    }
                                                    * match rows {
                                                        VectorSize::Bi => 2,
                                                        VectorSize::Tri => 3,
                                                        VectorSize::Quad => 4,
                                                    }
                                            }
                                            TypeInner::Struct { span, .. } => span,
                                            _ => panic!("Invalid Type"),
                                        };

                                        (handle, span)
                                    }
                                };
                                members.push(StructMember {
                                    name: Some(next.id.name().to_string()),
                                    ty: member,
                                    binding: None,
                                    offset: total_span,
                                });
                                (members, total_span + span)
                            });

                    NagaType {
                        name: Some(def.name().to_string()),
                        inner: TypeInner::Struct { members, span },
                    }
                }
            };

            self.types.insert(ty, Span::UNDEFINED);
        }
    }

    fn body_mut(&mut self) -> &mut NagaBlock {
        #[cfg(feature = "print")]
        println!("body_mut");

        self.block_stack
            .last_mut()
            .expect("Not inside a function body")
    }

    fn queue_mut(&mut self) -> &mut Vec<Handle<Expression>> {
        #[cfg(feature = "print")]
        println!("queue_mut");

        &mut self
            .expressions
            .as_mut()
            .expect("Not inside a function body")
            .queue
    }

    fn expressions_mut(&mut self) -> &mut Arena<Expression> {
        #[cfg(feature = "print")]
        println!("expressions_mut");

        &mut self
            .expressions
            .as_mut()
            .expect("Not inside a function body")
            .expressions
    }

    fn local_variables_mut(&mut self) -> &mut Arena<LocalVariable> {
        #[cfg(feature = "print")]
        println!("local_variables_mut");

        &mut self
            .local_variables
            .as_mut()
            .expect("Not inside a function body")
            .locals
    }

    fn pointers_mut(&mut self) -> &mut IndexMap<Handle<LocalVariable>, Handle<Expression>> {
        #[cfg(feature = "print")]
        println!("pointers_mut");

        &mut self
            .local_variables
            .as_mut()
            .expect("Not inside a function body")
            .pointers
    }

    fn push_statement(&mut self, stmt: Statement) {
        #[cfg(feature = "print")]
        println!("push_statement");

        self.flush_expressions();
        self.body_mut().push(stmt, Span::UNDEFINED);
    }

    fn push_expression(&mut self, expr: Expression) -> Handle<Expression> {
        #[cfg(feature = "print")]
        println!("push_expression");

        let push = match &expr {
            Expression::LocalVariable { .. } => false,
            Expression::FunctionArgument { .. } => false,
            Expression::CallResult { .. } => false,
            Expression::Literal { .. } => false,
            _ => true,
        };

        let handle = self.expressions_mut().append(expr, Span::UNDEFINED);

        if push {
            self.queue_mut().push(handle);
        } else {
            self.flush_expressions();
        }

        handle
    }

    fn push_type(&mut self, ty: NagaType) -> Handle<NagaType> {
        #[cfg(feature = "print")]
        println!("push_type");

        self.types.insert(ty, Span::UNDEFINED)
    }

    fn flush_expressions(&mut self) {
        #[cfg(feature = "print")]
        println!("flush_expressions");

        let mut iter = self.queue_mut().drain(..);
        if let Some(first) = iter.next() {
            let last = iter.last().unwrap_or_else(|| first);
            self.push_statement(Statement::Emit(Range::new_from_bounds(first, last)));
        }
    }

    fn push_local_variable(
        &mut self,
        local: LocalVariable,
    ) -> (Handle<LocalVariable>, Handle<Expression>) {
        #[cfg(feature = "print")]
        println!("push_local_variable");

        let local = self.local_variables_mut().append(local, Span::UNDEFINED);
        let pointer = self.push_expression(Expression::LocalVariable(local));
        self.pointers_mut().insert(local, pointer);
        (local, pointer)
    }

    fn get_input_type(&self, id: &PropertyIdentifier) -> &elysian_core::ir::module::Type {
        #[cfg(feature = "print")]
        println!("get_input_type");

        self.input
            .props
            .get(id)
            .unwrap_or_else(|| panic!("No input type for {}", id.name()))
    }

    fn functions_to_naga(&mut self) {
        #[cfg(feature = "print")]
        println!("functions_to_naga");

        let builtins = vec![
            elysian_function! {
                fn SAFE_NORMALIZE_2(VECTOR2) -> VECTOR2 {
                    if VECTOR2.length() > 0.0 {
                        return VECTOR2.normalize();
                    }

                    return VECTOR2;
                }
            },
            elysian_function! {
                fn SAFE_NORMALIZE_3(VECTOR3) -> VECTOR3 {
                    if VECTOR3.length() > 0.0 {
                        return VECTOR3.normalize();
                    }

                    return VECTOR3;
                }
            },
            elysian_function! {
                fn SAFE_NORMALIZE_4(VECTOR4) -> VECTOR4 {
                    if VECTOR4.length() > 0.0 {
                        return VECTOR4.normalize();
                    }

                    return VECTOR4;
                }
            },
        ];

        let handles = builtins
            .iter()
            .chain(self.input.function_definitions.iter())
            .map(|def| {
                (
                    self.functions.append(
                        Function {
                            name: Some(def.name_unique()),
                            arguments: def
                                .inputs
                                .iter()
                                .map(|input| FunctionArgument {
                                    name: Some(input.id.name().to_string()),
                                    ty: self.get_type(self.get_input_type(&input.id).name()).0,
                                    binding: None,
                                })
                                .collect(),
                            result: Some(FunctionResult {
                                ty: self.get_type(def.output.name()).0,
                                binding: None,
                            }),
                            local_variables: Default::default(),
                            expressions: Default::default(),
                            named_expressions: Default::default(),
                            body: Default::default(),
                        },
                        Span::UNDEFINED,
                    ),
                    def,
                )
            })
            .collect::<IndexMap<_, _>>();

        for (handle, def) in handles {
            self.block_stack.push(NagaBlock::new());

            self.expressions = Some(ExpressionQueue::default());
            self.local_variables = Some(LocalVariableStore::default());

            for (i, input_def) in def.inputs.iter().enumerate() {
                let (_, local_ptr) = self.push_local_variable(LocalVariable {
                    name: Some(input_def.id.name().to_string()),
                    ty: self.get_type(self.get_input_type(&input_def.id).name()).0,
                    init: None,
                });

                let value = self.push_expression(Expression::FunctionArgument(i as u32));

                self.push_statement(Statement::Store {
                    pointer: local_ptr,
                    value,
                });
            }

            self.function = Some(def.clone());
            for stmt in def.block.0.iter() {
                self.stmt_to_naga(stmt);
            }
            self.function = None;

            let f = self.functions.get_mut(handle);
            f.expressions = self.expressions.take().unwrap().expressions;
            f.local_variables = self.local_variables.take().unwrap().locals;
            f.body = self.block_stack.pop().unwrap();
        }
    }

    fn access_index(
        &self,
        base: Handle<Expression>,
        prev: &PropertyIdentifier,
        next: &PropertyIdentifier,
    ) -> Expression {
        #[cfg(feature = "print")]
        println!("access_index");

        Expression::AccessIndex {
            base,
            index: match self.get_input_type(prev) {
                ElysianType::Struct(s) => self
                    .input
                    .struct_definitions
                    .iter()
                    .find(|cand| cand.id == *s)
                    .unwrap_or_else(|| panic!("No struct definition for {}", prev.name()))
                    .fields
                    .iter()
                    .position(|field| field.id == *next)
                    .unwrap_or_else(|| panic!("No field {next:#?} for struct {s:#?}"))
                    as u32,
                t => panic!("Not a struct: {t:#?}"),
            },
        }
    }

    fn stmt_to_naga(&mut self, stmt: &Stmt) {
        #[cfg(feature = "print")]
        println!("stmt_to_naga");

        match stmt {
            Stmt::Block(block) => block.0.iter().for_each(|t| self.stmt_to_naga(t)),
            Stmt::Bind { prop, expr } => {
                let local_ptr = if let Some(k) = self.get_pointer(prop.name()) {
                    k.clone()
                } else {
                    let (_, local_ptr) = self.push_local_variable(naga::LocalVariable {
                        name: Some(prop.name().to_string()),
                        ty: self.get_type(self.get_input_type(prop).name()).0,
                        init: None,
                    });
                    local_ptr
                };

                let value = self.expr_to_naga(expr);

                self.push_statement(Statement::Store {
                    pointer: local_ptr,
                    value,
                })
            }
            Stmt::Write { path, expr } => {
                let mut iter = path.iter();

                let base = iter.next().unwrap();

                let base_expr = if let Some(base_expr) = self.get_pointer(base.name()) {
                    base_expr.clone()
                } else {
                    panic!("Invalid write")
                };

                let (_, pointer) = iter.fold((base.clone(), base_expr), |(prev, expr), next| {
                    let expr = self.push_expression(self.access_index(expr, &prev, next));

                    (next.clone(), expr)
                });

                let value = self.expr_to_naga(expr);

                self.push_statement(Statement::Store { pointer, value })
            }
            Stmt::If {
                cond,
                then,
                otherwise,
            } => {
                let condition = self.expr_to_naga(cond);
                self.flush_expressions();

                self.block_stack.push(NagaBlock::default());
                self.stmt_to_naga(then);
                let accept = self.block_stack.pop().unwrap();

                self.block_stack.push(NagaBlock::default());
                if let Some(otherwise) = otherwise {
                    self.stmt_to_naga(otherwise)
                };
                let reject = self.block_stack.pop().unwrap();

                self.push_statement(Statement::If {
                    condition,
                    accept,
                    reject,
                })
            }
            Stmt::Loop { stmt } => {
                self.block_stack.push(NagaBlock::default());
                self.stmt_to_naga(stmt);
                let loop_body = self.block_stack.pop().unwrap();

                self.push_statement(Statement::Loop {
                    body: loop_body,
                    continuing: Default::default(),
                    break_if: None,
                })
            }
            Stmt::Break => self.push_statement(Statement::Break),
            Stmt::Output(expr) => {
                let value = self.expr_to_naga(expr);
                self.push_statement(Statement::Return { value: Some(value) });
            }
        }
    }

    fn naga_default(&self, ty: &ElysianType) -> Value {
        #[cfg(feature = "print")]
        println!("naga_default");

        match ty {
            ElysianType::Boolean => Value::Boolean(false),
            ElysianType::Number(n) => match n {
                NumericType::UInt => Value::Number(Number::UInt(0)),
                NumericType::SInt => Value::Number(Number::SInt(0)),
                NumericType::Float => Value::Number(Number::Float(0.0)),
            },
            ElysianType::Struct(s) => {
                let mut out = Struct::new(s.clone());
                for field in self
                    .input
                    .struct_definitions
                    .iter()
                    .find(|cand| cand.id == *s)
                    .unwrap()
                    .fields
                    .iter()
                {
                    out.set_mut(
                        field.id.clone(),
                        self.naga_default(self.get_input_type(&field.id)),
                    );
                }
                Value::Struct(out)
            }
        }
    }

    fn expr_to_naga(&mut self, expr: &Expr) -> Handle<Expression> {
        #[cfg(feature = "print")]
        println!("expr_to_naga");

        match expr {
            Expr::Literal(v) => self.value_to_naga(v),
            Expr::Struct(def, members) => {
                let components = self
                    .input
                    .struct_definitions
                    .iter()
                    .find(|cand| cand.id == *def)
                    .unwrap()
                    .fields
                    .iter()
                    .map(|field| -> Expr {
                        if let Some(member) = members.get(&field.id) {
                            member.clone()
                        } else {
                            Expr::Literal(self.naga_default(&self.get_input_type(&field.id)))
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|member| self.expr_to_naga(&member))
                    .collect();

                let expr = self.push_expression(Expression::Compose {
                    ty: self.get_type(def.name()).0,
                    components,
                });

                expr
            }
            Expr::Read(path) => {
                let mut iter = path.iter();

                let base = iter.next().unwrap();

                let base_expr = if let Some(pointer) = self.get_pointer(base.name()) {
                    self.push_expression(Expression::Load {
                        pointer: pointer.clone(),
                    })
                } else {
                    panic!("Invalid Read")
                };

                let (_, expr) = iter.fold((base.clone(), base_expr), |(prev, expr), next| {
                    let access = self.push_expression(self.access_index(expr, &prev, next));
                    (next.clone(), access)
                });

                expr
            }
            Expr::Call {
                function: func,
                args,
            } => {
                let f = self.get_function(&func.name_unique()).0;

                let arguments = args.into_iter().map(|arg| self.expr_to_naga(arg)).collect();

                let expr = self.push_expression(Expression::CallResult(f));

                self.push_statement(Statement::Call {
                    function: f,
                    arguments,
                    result: Some(expr),
                });

                expr
            }
            Expr::Neg(t) => {
                let expr = self.expr_to_naga(t);

                let expr = self.push_expression(Expression::Unary {
                    op: naga::UnaryOperator::Negate,
                    expr,
                });

                expr
            }
            Expr::Add(lhs, rhs)
            | Expr::Sub(lhs, rhs)
            | Expr::Mul(lhs, rhs)
            | Expr::Div(lhs, rhs)
            | Expr::Eq(lhs, rhs)
            | Expr::Ne(lhs, rhs)
            | Expr::Lt(lhs, rhs)
            | Expr::Gt(lhs, rhs)
            | Expr::And(lhs, rhs)
            | Expr::Or(lhs, rhs) => {
                let type_l = lhs.ty(&self.input.function_definitions, &self.input.props);
                let type_r = rhs.ty(&self.input.function_definitions, &self.input.props);

                let invalid = |a, b| format!("Invalid Binary Op {}, {}", a, b);

                let (lhs, rhs) = match (type_l, type_r) {
                    (ElysianType::Boolean, ElysianType::Boolean) => (lhs, rhs),
                    (ElysianType::Number(a), ElysianType::Number(b)) => {
                        if a == b {
                            (lhs, rhs)
                        } else {
                            panic!("{}", invalid(a.name(), b.name()))
                        }
                    }
                    (ElysianType::Number(..), ElysianType::Struct(s))
                    | (ElysianType::Struct(s), ElysianType::Number(..)) => match expr {
                        Expr::Mul(_, _) => match s.name() {
                            "Vector2" | "Vector3" | "Vector4" | "Matrix2" | "Matrix3"
                            | "Matrix4" => (lhs, rhs),
                            _ => panic!("Invalid Binary Op"),
                        },
                        _ => panic!("Invalid Binary Op"),
                    },
                    (ElysianType::Struct(a), ElysianType::Struct(b)) => match expr {
                        Expr::Add(_, _) => match (a.name(), b.name()) {
                            ("Vector2", "Vector2")
                            | ("Vector3", "Vector3")
                            | ("Vector4", "Vector4")
                            | ("Matrix2", "Matrix2")
                            | ("Matrix3", "Matrix3")
                            | ("Matrix4", "Matrix4") => (lhs, rhs),
                            _ => panic!("{}", invalid(a.name(), b.name())),
                        },
                        Expr::Sub(_, _) => match (a.name(), b.name()) {
                            ("Vector2", "Vector2")
                            | ("Vector3", "Vector3")
                            | ("Vector4", "Vector4")
                            | ("Matrix2", "Matrix2")
                            | ("Matrix3", "Matrix3")
                            | ("Matrix4", "Matrix4") => (lhs, rhs),
                            _ => panic!("{}", invalid(a.name(), b.name())),
                        },
                        Expr::Mul(_, _) => match (a.name(), b.name()) {
                            ("Vector2", "Vector2")
                            | ("Vector3", "Vector3")
                            | ("Vector4", "Vector4")
                            | ("Matrix2", "Matrix2")
                            | ("Matrix3", "Matrix3")
                            | ("Matrix4", "Matrix4")
                            | ("Vector2", "Matrix2")
                            | ("Vector3", "Matrix3")
                            | ("Vector4", "Matrix4")
                            | ("Matrix2", "Vector2")
                            | ("Matrix3", "Vector3")
                            | ("Matrix4", "Vector4") => (lhs, rhs),
                            _ => panic!("{}", invalid(a.name(), b.name())),
                        },
                        Expr::Div(_, _) => match (a.name(), b.name()) {
                            ("Vector2", "Vector2")
                            | ("Vector3", "Vector3")
                            | ("Vector4", "Vector4") => (lhs, rhs),
                            _ => panic!("{}", invalid(a.name(), b.name())),
                        },
                        Expr::Eq(_, _) | Expr::Ne(_, _) | Expr::Lt(_, _) | Expr::Gt(_, _) => {
                            if a == b {
                                (lhs, rhs)
                            } else {
                                panic!("{}", invalid(a.name(), b.name()))
                            }
                        }
                        _ => unreachable!(),
                    },
                    _ => panic!("Invalid Binary Op"),
                };

                let left = self.expr_to_naga(lhs);
                let right = self.expr_to_naga(rhs);

                self.push_expression(Expression::Binary {
                    op: match expr {
                        Expr::Add(..) => BinaryOperator::Add,
                        Expr::Sub(..) => BinaryOperator::Subtract,
                        Expr::Mul(..) => BinaryOperator::Multiply,
                        Expr::Div(..) => BinaryOperator::Divide,
                        Expr::Eq(..) => BinaryOperator::Equal,
                        Expr::Ne(..) => BinaryOperator::NotEqual,
                        Expr::Lt(..) => BinaryOperator::Less,
                        Expr::Gt(..) => BinaryOperator::Greater,
                        Expr::And(..) => BinaryOperator::LogicalAnd,
                        Expr::Or(..) => BinaryOperator::LogicalOr,
                        _ => unreachable!(),
                    },
                    left,
                    right,
                })
            }
            Expr::Min(lhs, rhs)
            | Expr::Max(lhs, rhs)
            | Expr::Dot(lhs, rhs)
            | Expr::Atan2(lhs, rhs) => {
                let arg = self.expr_to_naga(lhs);
                let arg1 = self.expr_to_naga(rhs);
                let expr = self.push_expression(Expression::Math {
                    fun: match expr {
                        Expr::Min(..) => MathFunction::Min,
                        Expr::Max(..) => MathFunction::Max,
                        Expr::Dot(..) => MathFunction::Dot,
                        Expr::Atan2(..) => MathFunction::Atan2,
                        _ => unreachable!(),
                    },
                    arg,
                    arg1: Some(arg1),
                    arg2: None,
                    arg3: None,
                });

                expr
            }
            Expr::Abs(t) | Expr::Sign(t) | Expr::Length(t) | Expr::Acos(t) | Expr::Atan(t) => {
                let arg = self.expr_to_naga(t);

                self.push_expression(Expression::Math {
                    fun: match expr {
                        Expr::Abs(..) => MathFunction::Abs,
                        Expr::Sign(..) => MathFunction::Sign,
                        Expr::Length(..) => MathFunction::Length,
                        Expr::Normalize(..) => MathFunction::Normalize,
                        Expr::Acos(..) => MathFunction::Acos,
                        Expr::Atan(..) => MathFunction::Atan,
                        _ => unreachable!(),
                    },
                    arg,
                    arg1: None,
                    arg2: None,
                    arg3: None,
                })
            }
            Expr::Normalize(t) => {
                let arg = self.expr_to_naga(t);

                let function_id = &self.function.as_ref().unwrap().id;
                if *function_id == SAFE_NORMALIZE_2
                    || *function_id == SAFE_NORMALIZE_3
                    || *function_id == SAFE_NORMALIZE_4
                {
                    self.push_expression(Expression::Math {
                        fun: MathFunction::Normalize,
                        arg,
                        arg1: None,
                        arg2: None,
                        arg3: None,
                    })
                } else {
                    let function =
                        match t.ty(&self.input.function_definitions, &self.input.props) {
                            ElysianType::Struct(s) => match s.name() {
                                "Vector2" => self.get_function(&SAFE_NORMALIZE_2.name_unique()),
                                "Vector3" => self.get_function(&SAFE_NORMALIZE_3.name_unique()),
                                "Vector4" => self.get_function(&SAFE_NORMALIZE_4.name_unique()),
                                _ => panic!("Invalid Normalize"),
                            },
                            _ => panic!("Invalid Normalize"),
                        }
                        .0;

                    let call_result = self.push_expression(Expression::CallResult(function));

                    self.push_statement(Statement::Call {
                        function,
                        arguments: vec![arg],
                        result: Some(call_result),
                    });

                    call_result
                }
            }
            Expr::Mix(lhs, rhs, t) => {
                let arg = self.expr_to_naga(lhs);
                let arg1 = self.expr_to_naga(rhs);
                let arg2 = self.expr_to_naga(t);

                let expr = self.push_expression(Expression::Math {
                    fun: MathFunction::Mix,
                    arg,
                    arg1: Some(arg1),
                    arg2: Some(arg2),
                    arg3: None,
                });

                expr
            }
        }
    }

    fn number_to_naga(number: &Number) -> Expression {
        #[cfg(feature = "print")]
        println!("number_to_naga");

        match number {
            Number::UInt(u) => Expression::Literal(Literal::U32(*u as u32)),
            Number::SInt(i) => Expression::Literal(Literal::I32(*i as i32)),
            Number::Float(f) => Expression::Literal(Literal::F32(*f as f32)),
        }
    }

    fn value_to_naga(&mut self, value: &Value) -> Handle<Expression> {
        #[cfg(feature = "print")]
        println!("value_to_naga");

        match value {
            Value::Boolean(b) => self.push_expression(Expression::Literal(Literal::Bool(*b))),
            Value::Number(n) => self.push_expression(Self::number_to_naga(n)),
            Value::Struct(s) => {
                let ty = self.get_type(s.id.name()).0;

                let mut components = vec![];

                for field in self
                    .input
                    .struct_definitions
                    .iter()
                    .find(|cand| cand.id == s.id)
                    .unwrap_or_else(|| panic!("No struct definition for {}", s.id.name()))
                    .fields
                    .iter()
                {
                    let v = s.get(&field.id);
                    let v = self.value_to_naga(&v);
                    components.push(v);
                }

                let expr = self.push_expression(Expression::Compose { ty, components });

                expr
            }
        }
    }

    fn shadertoy_entry_point(&mut self) -> EntryPoint {
        #[cfg(feature = "print")]
        println!("shadertoy_entry_point");

        let context_struct = self
            .input
            .struct_definitions
            .iter()
            .find(|cand| *cand.id == CONTEXT)
            .unwrap();

        self.block_stack.push(NagaBlock::new());
        self.expressions = Some(ExpressionQueue::default());
        self.local_variables = Some(LocalVariableStore::default());

        let (_, frag_color_ptr) = self.push_local_variable(LocalVariable {
            name: Some("fragColor".to_string()),
            ty: self.get_type("Vector4").0,
            init: None,
        });

        let (_, context_ptr) = self.push_local_variable(LocalVariable {
            name: Some("context".to_string()),
            ty: self.get_type("Context").0,
            init: None,
        });

        let frag_coord_arg = self.push_expression(Expression::FunctionArgument(0));
        let resolution_arg = self.push_expression(Expression::FunctionArgument(2));

        let resolution_xy = self.push_expression(Expression::Swizzle {
            size: VectorSize::Bi,
            vector: resolution_arg,
            pattern: SwizzleComponent::XYZW,
        });

        let uv_expr = self.push_expression(Expression::Binary {
            op: BinaryOperator::Divide,
            left: frag_coord_arg,
            right: resolution_xy,
        });

        let two = self.push_expression(Expression::Literal(Literal::F32(2.0)));

        let uv_expr = self.push_expression(Expression::Binary {
            op: BinaryOperator::Multiply,
            left: uv_expr,
            right: two,
        });

        let one = self.push_expression(Expression::Literal(Literal::F32(1.0)));

        let one_v2 = self.push_expression(Expression::Compose {
            ty: self.get_type("Vector2").0,
            components: vec![one, one],
        });

        let uv_expr = self.push_expression(Expression::Binary {
            op: BinaryOperator::Subtract,
            left: uv_expr,
            right: one_v2,
        });

        let uv_expr = self.push_expression(Expression::Binary {
            op: BinaryOperator::Multiply,
            left: uv_expr,
            right: two,
        });

        let entry_point = self.get_function(&self.input.entry_point.name_unique()).0;

        let position_2d = self.push_expression(Expression::AccessIndex {
            base: context_ptr,
            index: context_struct
                .fields
                .iter()
                .position(|field| *field.id == POSITION_2D)
                .unwrap() as u32,
        });

        self.push_statement(Statement::Store {
            pointer: position_2d,
            value: uv_expr,
        });

        let aspect = self.push_expression(Expression::AccessIndex {
            base: context_ptr,
            index: context_struct
                .fields
                .iter()
                .position(|field| *field.id == ASPECT)
                .expect("No aspect field") as u32,
        });

        let resolution_x = self.push_expression(Expression::AccessIndex {
            base: resolution_xy,
            index: 0,
        });

        let resolution_y = self.push_expression(Expression::AccessIndex {
            base: resolution_xy,
            index: 1,
        });

        let aspect_expr = self.push_expression(Expression::Binary {
            op: BinaryOperator::Divide,
            left: resolution_x,
            right: resolution_y,
        });

        self.push_statement(Statement::Store {
            pointer: aspect,
            value: aspect_expr,
        });

        let context = self.push_expression(Expression::Load {
            pointer: context_ptr,
        });

        let call_result = self.push_expression(Expression::CallResult(entry_point));

        self.push_statement(Statement::Call {
            function: entry_point,
            arguments: vec![context],
            result: Some(call_result),
        });

        self.push_statement(Statement::Store {
            pointer: context_ptr,
            value: call_result,
        });

        let color_ptr = self.push_expression(Expression::AccessIndex {
            base: context_ptr,
            index: context_struct
                .fields
                .iter()
                .position(|field| *field.id == COLOR)
                .unwrap() as u32,
        });

        let color = self.push_expression(Expression::Load { pointer: color_ptr });

        self.push_statement(Statement::Store {
            pointer: frag_color_ptr,
            value: color,
        });

        let expressions = self.expressions.take().unwrap().expressions;
        let local_variables = self.local_variables.take().unwrap().locals;
        let body = self.block_stack.pop().unwrap();

        EntryPoint {
            name: "mainImage".to_string(),
            stage: ShaderStage::Fragment,
            early_depth_test: None,
            workgroup_size: [0; 3],
            function: Function {
                name: Some("mainImage".to_string()),
                arguments: vec![
                    FunctionArgument {
                        name: Some("fragCoord".to_string()),
                        ty: self.get_type("Vector2").0,
                        binding: Some(naga::Binding::Location {
                            location: 0,
                            interpolation: Some(naga::Interpolation::Perspective),
                            sampling: None,
                        }),
                    },
                    FunctionArgument {
                        name: Some("fragColor".to_string()),
                        ty: self.get_type("Vector4").0,
                        binding: Some(naga::Binding::Location {
                            location: 2,
                            interpolation: Some(naga::Interpolation::Perspective),
                            sampling: None,
                        }),
                    },
                    FunctionArgument {
                        name: Some("iResolution".to_string()),
                        ty: self.get_type("Vector3").0,
                        binding: Some(naga::Binding::Location {
                            location: 1,
                            interpolation: Some(naga::Interpolation::Perspective),
                            sampling: None,
                        }),
                    },
                ],
                result: None,
                local_variables,
                expressions,
                named_expressions: Default::default(),
                body,
            },
        }
    }
}
