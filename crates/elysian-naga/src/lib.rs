use elysian_core::ir::{
    ast::{Expr, Number, Property, Stmt, Struct, Value},
    module::{Module as ElysianModule, NumericType, Type as ElysianType},
};
use indexmap::IndexMap;
use naga::{
    Arena, BinaryOperator, Block as NagaBlock, EntryPoint, Expression, Function, FunctionArgument,
    FunctionResult, Handle, Literal, LocalVariable, MathFunction, Module as NagaModule, Range,
    ScalarKind, ShaderStage, Span, Statement, StructMember, SwizzleComponent, Type as NagaType,
    TypeInner, UniqueArena, VectorSize,
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

pub struct NagaWriter<'a> {
    input: &'a ElysianModule,
    types: UniqueArena<NagaType>,
    functions: Arena<Function>,
    block_stack: Vec<NagaBlock>,
    expressions: Option<ExpressionQueue>,
    local_variables: Option<LocalVariableStore>,
}

impl<'a> NagaWriter<'a> {
    pub fn new(module: &'a ElysianModule) -> Self {
        NagaWriter {
            input: module,
            types: Default::default(),
            functions: Default::default(),
            block_stack: Default::default(),
            expressions: Default::default(),
            local_variables: Default::default(),
        }
    }

    pub fn module_to_naga(mut self) -> NagaModule {
        self.types_to_naga();
        self.functions_to_naga();
        let entry_point = self.shadertoy_entry_point();

        NagaModule {
            types: self.types,
            special_types: Default::default(),
            constants: Default::default(),
            global_variables: Default::default(),
            const_expressions: Default::default(),
            functions: self.functions,
            entry_points: vec![entry_point],
        }
    }

    fn get_type(&self, name: &str) -> (Handle<NagaType>, &NagaType) {
        self.types
            .iter()
            .find(|(_, v)| v.name.as_ref().map(String::as_str) == Some(name))
            .expect("No type")
    }

    fn get_function(&self, name: &str) -> (Handle<Function>, &Function) {
        self.functions
            .iter()
            .find(|(_, v)| v.name.as_ref().map(String::as_str) == Some(name))
            .unwrap_or_else(|| panic!("No function for {name:}"))
    }

    fn get_local_variable(&self, name: &str) -> Option<(Handle<LocalVariable>, &LocalVariable)> {
        self.local_variables
            .as_ref()?
            .locals
            .iter()
            .find(|(_, v)| v.name.as_ref().map(String::as_str) == Some(name))
    }

    fn get_pointer(&self, name: &str) -> Option<&Handle<Expression>> {
        let (k, _) = self.get_local_variable(name)?;
        self.local_variables
            .as_ref()
            .expect("Not in a block")
            .pointers
            .get(&k)
    }

    fn types_to_naga(&mut self) {
        let bool = self.push_type(NagaType {
            name: Some("Bool".to_string()),
            inner: TypeInner::Scalar {
                kind: ScalarKind::Bool,
                width: 1,
            },
        });

        let uint = self.push_type(NagaType {
            name: Some("u32".to_string()),
            inner: TypeInner::Scalar {
                kind: ScalarKind::Uint,
                width: 4,
            },
        });

        let sint = self.push_type(NagaType {
            name: Some("i32".to_string()),
            inner: TypeInner::Scalar {
                kind: ScalarKind::Sint,
                width: 4,
            },
        });

        let float = self.push_type(NagaType {
            name: Some("f32".to_string()),
            inner: TypeInner::Scalar {
                kind: ScalarKind::Float,
                width: 4,
            },
        });

        self.push_type(NagaType {
            name: Some("Vector2".to_string()),
            inner: TypeInner::Vector {
                size: VectorSize::Bi,
                kind: ScalarKind::Float,
                width: 4,
            },
        });

        self.push_type(NagaType {
            name: Some("Vector3".to_string()),
            inner: TypeInner::Vector {
                size: VectorSize::Tri,
                kind: ScalarKind::Float,
                width: 4,
            },
        });

        self.push_type(NagaType {
            name: Some("Vector4".to_string()),
            inner: TypeInner::Vector {
                size: VectorSize::Quad,
                kind: ScalarKind::Float,
                width: 4,
            },
        });

        self.push_type(NagaType {
            name: Some("Matrix2".to_string()),
            inner: TypeInner::Matrix {
                columns: VectorSize::Bi,
                rows: VectorSize::Bi,
                width: 4,
            },
        });

        self.push_type(NagaType {
            name: Some("Matrix3".to_string()),
            inner: TypeInner::Matrix {
                columns: VectorSize::Tri,
                rows: VectorSize::Tri,
                width: 4,
            },
        });

        self.push_type(NagaType {
            name: Some("Matrix4".to_string()),
            inner: TypeInner::Matrix {
                columns: VectorSize::Quad,
                rows: VectorSize::Quad,
                width: 4,
            },
        });

        for def in &self.input.struct_definitions {
            let (members, span) =
                def.fields
                    .iter()
                    .fold((vec![], 0), |(mut members, total_span), next| {
                        let (member, span) = match next.prop.ty() {
                            ElysianType::Boolean => (bool, 1),
                            ElysianType::Number(n) => match n {
                                NumericType::UInt => (uint, 4),
                                NumericType::SInt => (sint, 4),
                                NumericType::Float => (float, 4),
                            },
                            ElysianType::Struct(s) => {
                                let (handle, ty) = self.get_type(s.id.name());

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
                            name: Some(next.prop.name().to_string()),
                            ty: member,
                            binding: None,
                            offset: total_span,
                        });
                        (members, total_span + span)
                    });

            let ty = NagaType {
                name: Some(def.name().to_string()),
                inner: TypeInner::Struct { members, span },
            };

            self.types.insert(ty, Span::UNDEFINED);
        }
    }

    fn body_mut(&mut self) -> &mut NagaBlock {
        self.block_stack
            .last_mut()
            .expect("Not inside a function body")
    }

    fn queue_mut(&mut self) -> &mut Vec<Handle<Expression>> {
        &mut self
            .expressions
            .as_mut()
            .expect("Not inside a function body")
            .queue
    }

    fn expressions_mut(&mut self) -> &mut Arena<Expression> {
        &mut self
            .expressions
            .as_mut()
            .expect("Not inside a function body")
            .expressions
    }

    fn local_variables_mut(&mut self) -> &mut Arena<LocalVariable> {
        &mut self
            .local_variables
            .as_mut()
            .expect("Not inside a function body")
            .locals
    }

    fn pointers_mut(&mut self) -> &mut IndexMap<Handle<LocalVariable>, Handle<Expression>> {
        &mut self
            .local_variables
            .as_mut()
            .expect("Not inside a function body")
            .pointers
    }

    fn push_statement(&mut self, stmt: Statement) {
        self.flush_expressions();
        self.body_mut().push(stmt, Span::UNDEFINED);
    }

    fn push_expression(&mut self, expr: Expression) -> Handle<Expression> {
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
        self.types.insert(ty, Span::UNDEFINED)
    }

    fn flush_expressions(&mut self) {
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
        let local = self.local_variables_mut().append(local, Span::UNDEFINED);
        let pointer = self.push_expression(Expression::LocalVariable(local));
        self.pointers_mut().insert(local, pointer);
        (local, pointer)
    }

    fn functions_to_naga(&mut self) {
        let handles = self
            .input
            .function_definitions
            .iter()
            .map(|def| {
                (
                    self.functions.append(
                        Function {
                            name: Some(def.name_unique()),
                            arguments: def
                                .inputs
                                .iter()
                                .map(|input| FunctionArgument {
                                    name: Some(input.prop.name().to_string()),
                                    ty: self.get_type(input.prop.ty().name()).0,
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
                    name: Some(input_def.prop.name().to_string()),
                    ty: self.get_type(input_def.prop.ty().name()).0,
                    init: None,
                });

                let value = self.push_expression(Expression::FunctionArgument(i as u32));

                self.push_statement(Statement::Store {
                    pointer: local_ptr,
                    value,
                });
            }

            for stmt in def.block.0.iter() {
                self.stmt_to_naga(stmt);
            }

            let f = self.functions.get_mut(handle);
            f.expressions = self.expressions.take().unwrap().expressions;
            f.local_variables = self.local_variables.take().unwrap().locals;
            f.body = self.block_stack.pop().unwrap();
        }
    }

    fn access_index(base: Handle<Expression>, prev: &Property, next: &Property) -> Expression {
        Expression::AccessIndex {
            base,
            index: match prev.ty() {
                ElysianType::Struct(s) => s
                    .fields
                    .iter()
                    .position(|field| field.prop == *next)
                    .unwrap_or_else(|| panic!("No field {next:#?} for struct {s:#?}"))
                    as u32,
                t => panic!("Not a struct: {t:#?}"),
            },
        }
    }

    fn stmt_to_naga(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block(block) => block.0.iter().for_each(|t| self.stmt_to_naga(t)),
            Stmt::Bind { prop, expr } => {
                let local_ptr = if let Some(k) = self.get_pointer(prop.name()) {
                    k.clone()
                } else {
                    let (_, local_ptr) = self.push_local_variable(naga::LocalVariable {
                        name: Some(prop.name().to_string()),
                        ty: self.get_type(prop.ty().name()).0,
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
                    let expr = self.push_expression(Self::access_index(expr, &prev, next));

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

    fn naga_default(ty: &ElysianType) -> Value {
        match ty {
            ElysianType::Boolean => Value::Boolean(false),
            ElysianType::Number(n) => match n {
                NumericType::UInt => Value::Number(Number::UInt(0)),
                NumericType::SInt => Value::Number(Number::SInt(0)),
                NumericType::Float => Value::Number(Number::Float(0.0)),
            },
            ElysianType::Struct(s) => {
                let mut out = Struct::new(s);
                for field in s.fields {
                    out.set_mut(field.prop.clone(), Self::naga_default(field.prop.ty()));
                }
                Value::Struct(out)
            }
        }
    }

    fn expr_to_naga(&mut self, expr: &Expr) -> Handle<Expression> {
        match expr {
            Expr::Literal(v) => self.value_to_naga(v),
            Expr::Struct(def, members) => {
                let components = def
                    .fields
                    .iter()
                    .map(|field| -> Expr {
                        if let Some(member) = members.get(&field.prop) {
                            member.clone()
                        } else {
                            Expr::Literal(Self::naga_default(field.prop.ty()))
                        }
                    })
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
                    let access = self.push_expression(Self::access_index(expr, &prev, next));
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
            | Expr::Lt(lhs, rhs)
            | Expr::Gt(lhs, rhs) => {
                let type_l = lhs.ty();
                let type_r = rhs.ty();

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
                        Expr::Lt(_, _) => todo!(),
                        Expr::Gt(_, _) => todo!(),
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
                        Expr::Lt(..) => BinaryOperator::Less,
                        Expr::Gt(..) => BinaryOperator::Greater,
                        _ => unreachable!(),
                    },
                    left,
                    right,
                })
            }
            Expr::Min(lhs, rhs) | Expr::Max(lhs, rhs) | Expr::Dot(lhs, rhs) => {
                let arg = self.expr_to_naga(lhs);
                let arg1 = self.expr_to_naga(rhs);
                let expr = self.push_expression(Expression::Math {
                    fun: match expr {
                        Expr::Min(..) => MathFunction::Min,
                        Expr::Max(..) => MathFunction::Max,
                        Expr::Dot(..) => MathFunction::Dot,
                        _ => unreachable!(),
                    },
                    arg,
                    arg1: Some(arg1),
                    arg2: None,
                    arg3: None,
                });

                expr
            }
            Expr::Abs(t) | Expr::Sign(t) | Expr::Length(t) | Expr::Normalize(t) => {
                let arg = self.expr_to_naga(t);

                let expr = self.push_expression(Expression::Math {
                    fun: match expr {
                        Expr::Abs(..) => MathFunction::Abs,
                        Expr::Sign(..) => MathFunction::Sign,
                        Expr::Length(..) => MathFunction::Length,
                        Expr::Normalize(..) => MathFunction::Normalize,
                        _ => unreachable!(),
                    },
                    arg,
                    arg1: None,
                    arg2: None,
                    arg3: None,
                });

                expr
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
        match number {
            Number::UInt(u) => Expression::Literal(Literal::U32(*u as u32)),
            Number::SInt(i) => Expression::Literal(Literal::I32(*i as i32)),
            Number::Float(f) => Expression::Literal(Literal::F32(*f as f32)),
        }
    }

    fn value_to_naga(&mut self, value: &Value) -> Handle<Expression> {
        match value {
            Value::Boolean(b) => self.push_expression(Expression::Literal(Literal::Bool(*b))),
            Value::Number(n) => self.push_expression(Self::number_to_naga(n)),
            Value::Struct(s) => {
                let ty = self.get_type(s.def.name()).0;

                let mut components = vec![];

                for field in s.def.fields {
                    let v = s.get(&field.prop);
                    let v = self.value_to_naga(&v);
                    components.push(v);
                }

                let expr = self.push_expression(Expression::Compose { ty, components });

                expr
            }
        }
    }

    fn shadertoy_entry_point(&mut self) -> EntryPoint {
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
            index: 0,
        });

        self.push_statement(Statement::Store {
            pointer: position_2d,
            value: uv_expr,
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
            index: 10,
        });

        let color = self.push_expression(Expression::Load {
            pointer: color_ptr,
        });

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
