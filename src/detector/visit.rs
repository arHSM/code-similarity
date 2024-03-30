/* auto-generated with `bun scripts/generate.ts > src/detector/visit.rs`, swc @ 10ece99e0f28ccd6cfe0942e36d913721dbd3961 */

impl swc_ecma_visit::Visit for JsHasher {
    fn visit_class(&mut self, n: &swc_ecma_ast::Class) {
        self.hasher.write(b"Class");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_class_member(&mut self, n: &swc_ecma_ast::ClassMember) {
        self.hasher.write(b"ClassMember");
        match n {
            swc_ecma_ast::ClassMember::Constructor(_) => self.hasher.write(b"Constructor(_)"),
            swc_ecma_ast::ClassMember::Method(_) => self.hasher.write(b"Method(_)"),
            swc_ecma_ast::ClassMember::PrivateMethod(_) => self.hasher.write(b"PrivateMethod(_)"),
            swc_ecma_ast::ClassMember::ClassProp(_) => self.hasher.write(b"ClassProp(_)"),
            swc_ecma_ast::ClassMember::PrivateProp(_) => self.hasher.write(b"PrivateProp(_)"),
            swc_ecma_ast::ClassMember::TsIndexSignature(_) => self.hasher.write(b"TsIndexSignature(_)"),
            swc_ecma_ast::ClassMember::Empty(_) => self.hasher.write(b"Empty(_)"),
            swc_ecma_ast::ClassMember::StaticBlock(_) => self.hasher.write(b"StaticBlock(_)"),
            swc_ecma_ast::ClassMember::AutoAccessor(_) => self.hasher.write(b"AutoAccessor(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_class_prop(&mut self, n: &swc_ecma_ast::ClassProp) {
        self.hasher.write(b"ClassProp");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_private_prop(&mut self, n: &swc_ecma_ast::PrivateProp) {
        self.hasher.write(b"PrivateProp");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_class_method(&mut self, n: &swc_ecma_ast::ClassMethod) {
        self.hasher.write(b"ClassMethod");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_private_method(&mut self, n: &swc_ecma_ast::PrivateMethod) {
        self.hasher.write(b"PrivateMethod");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_constructor(&mut self, n: &swc_ecma_ast::Constructor) {
        self.hasher.write(b"Constructor");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_decorator(&mut self, n: &swc_ecma_ast::Decorator) {
        self.hasher.write(b"Decorator");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_static_block(&mut self, n: &swc_ecma_ast::StaticBlock) {
        self.hasher.write(b"StaticBlock");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_method_kind(&mut self, n: &swc_ecma_ast::MethodKind) {
        self.hasher.write(b"MethodKind");
        match n {
            swc_ecma_ast::MethodKind::Method => self.hasher.write(b"Method"),
            swc_ecma_ast::MethodKind::Getter => self.hasher.write(b"Getter"),
            swc_ecma_ast::MethodKind::Setter => self.hasher.write(b"Setter"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_decl(&mut self, n: &swc_ecma_ast::Decl) {
        self.hasher.write(b"Decl");
        match n {
            swc_ecma_ast::Decl::Class(_) => self.hasher.write(b"Class(_)"),
            swc_ecma_ast::Decl::Fn(_) => self.hasher.write(b"Fn(_)"),
            swc_ecma_ast::Decl::Var(_) => self.hasher.write(b"Var(_)"),
            swc_ecma_ast::Decl::Using(_) => self.hasher.write(b"Using(_)"),
            swc_ecma_ast::Decl::TsInterface(_) => self.hasher.write(b"TsInterface(_)"),
            swc_ecma_ast::Decl::TsTypeAlias(_) => self.hasher.write(b"TsTypeAlias(_)"),
            swc_ecma_ast::Decl::TsEnum(_) => self.hasher.write(b"TsEnum(_)"),
            swc_ecma_ast::Decl::TsModule(_) => self.hasher.write(b"TsModule(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_fn_decl(&mut self, n: &swc_ecma_ast::FnDecl) {
        self.hasher.write(b"FnDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_class_decl(&mut self, n: &swc_ecma_ast::ClassDecl) {
        self.hasher.write(b"ClassDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_var_decl(&mut self, n: &swc_ecma_ast::VarDecl) {
        self.hasher.write(b"VarDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_var_decl_kind(&mut self, n: &swc_ecma_ast::VarDeclKind) {
        self.hasher.write(b"VarDeclKind");
        match n {
            swc_ecma_ast::VarDeclKind::Var => self.hasher.write(b"Var"),
            swc_ecma_ast::VarDeclKind::Let => self.hasher.write(b"Let"),
            swc_ecma_ast::VarDeclKind::Const => self.hasher.write(b"Const"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_var_declarator(&mut self, n: &swc_ecma_ast::VarDeclarator) {
        self.hasher.write(b"VarDeclarator");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_expr(&mut self, n: &swc_ecma_ast::Expr) {
        self.hasher.write(b"Expr");
        match n {
            swc_ecma_ast::Expr::This(_) => self.hasher.write(b"This(_)"),
            swc_ecma_ast::Expr::Array(_) => self.hasher.write(b"Array(_)"),
            swc_ecma_ast::Expr::Object(_) => self.hasher.write(b"Object(_)"),
            swc_ecma_ast::Expr::Fn(_) => self.hasher.write(b"Fn(_)"),
            swc_ecma_ast::Expr::Unary(_) => self.hasher.write(b"Unary(_)"),
            swc_ecma_ast::Expr::Update(_) => self.hasher.write(b"Update(_)"),
            swc_ecma_ast::Expr::Bin(_) => self.hasher.write(b"Bin(_)"),
            swc_ecma_ast::Expr::Assign(_) => self.hasher.write(b"Assign(_)"),
            swc_ecma_ast::Expr::Member(_) => self.hasher.write(b"Member(_)"),
            swc_ecma_ast::Expr::SuperProp(_) => self.hasher.write(b"SuperProp(_)"),
            swc_ecma_ast::Expr::Cond(_) => self.hasher.write(b"Cond(_)"),
            swc_ecma_ast::Expr::Call(_) => self.hasher.write(b"Call(_)"),
            swc_ecma_ast::Expr::New(_) => self.hasher.write(b"New(_)"),
            swc_ecma_ast::Expr::Seq(_) => self.hasher.write(b"Seq(_)"),
            swc_ecma_ast::Expr::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::Expr::Lit(_) => self.hasher.write(b"Lit(_)"),
            swc_ecma_ast::Expr::Tpl(_) => self.hasher.write(b"Tpl(_)"),
            swc_ecma_ast::Expr::TaggedTpl(_) => self.hasher.write(b"TaggedTpl(_)"),
            swc_ecma_ast::Expr::Arrow(_) => self.hasher.write(b"Arrow(_)"),
            swc_ecma_ast::Expr::Class(_) => self.hasher.write(b"Class(_)"),
            swc_ecma_ast::Expr::Yield(_) => self.hasher.write(b"Yield(_)"),
            swc_ecma_ast::Expr::MetaProp(_) => self.hasher.write(b"MetaProp(_)"),
            swc_ecma_ast::Expr::Await(_) => self.hasher.write(b"Await(_)"),
            swc_ecma_ast::Expr::Paren(_) => self.hasher.write(b"Paren(_)"),
            swc_ecma_ast::Expr::JSXMember(_) => self.hasher.write(b"JSXMember(_)"),
            swc_ecma_ast::Expr::JSXNamespacedName(_) => self.hasher.write(b"JSXNamespacedName(_)"),
            swc_ecma_ast::Expr::JSXEmpty(_) => self.hasher.write(b"JSXEmpty(_)"),
            swc_ecma_ast::Expr::JSXElement(_) => self.hasher.write(b"JSXElement(_)"),
            swc_ecma_ast::Expr::JSXFragment(_) => self.hasher.write(b"JSXFragment(_)"),
            swc_ecma_ast::Expr::TsTypeAssertion(_) => self.hasher.write(b"TsTypeAssertion(_)"),
            swc_ecma_ast::Expr::TsConstAssertion(_) => self.hasher.write(b"TsConstAssertion(_)"),
            swc_ecma_ast::Expr::TsNonNull(_) => self.hasher.write(b"TsNonNull(_)"),
            swc_ecma_ast::Expr::TsAs(_) => self.hasher.write(b"TsAs(_)"),
            swc_ecma_ast::Expr::TsSatisfies(_) => self.hasher.write(b"TsSatisfies(_)"),
            swc_ecma_ast::Expr::TsInstantiation(_) => self.hasher.write(b"TsInstantiation(_)"),
            swc_ecma_ast::Expr::PrivateName(_) => self.hasher.write(b"PrivateName(_)"),
            swc_ecma_ast::Expr::OptChain(_) => self.hasher.write(b"OptChain(_)"),
            swc_ecma_ast::Expr::Invalid(_) => self.hasher.write(b"Invalid(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_this_expr(&mut self, n: &swc_ecma_ast::ThisExpr) {
        self.hasher.write(b"ThisExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_array_lit(&mut self, n: &swc_ecma_ast::ArrayLit) {
        self.hasher.write(b"ArrayLit");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_object_lit(&mut self, n: &swc_ecma_ast::ObjectLit) {
        self.hasher.write(b"ObjectLit");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_prop_or_spread(&mut self, n: &swc_ecma_ast::PropOrSpread) {
        self.hasher.write(b"PropOrSpread");
        match n {
            swc_ecma_ast::PropOrSpread::Spread(_) => self.hasher.write(b"Spread(_)"),
            swc_ecma_ast::PropOrSpread::Prop(_) => self.hasher.write(b"Prop(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_spread_element(&mut self, n: &swc_ecma_ast::SpreadElement) {
        self.hasher.write(b"SpreadElement");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_unary_expr(&mut self, n: &swc_ecma_ast::UnaryExpr) {
        self.hasher.write(b"UnaryExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_update_expr(&mut self, n: &swc_ecma_ast::UpdateExpr) {
        self.hasher.write(b"UpdateExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_bin_expr(&mut self, n: &swc_ecma_ast::BinExpr) {
        self.hasher.write(b"BinExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_fn_expr(&mut self, n: &swc_ecma_ast::FnExpr) {
        self.hasher.write(b"FnExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_class_expr(&mut self, n: &swc_ecma_ast::ClassExpr) {
        self.hasher.write(b"ClassExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_assign_expr(&mut self, n: &swc_ecma_ast::AssignExpr) {
        self.hasher.write(b"AssignExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_member_expr(&mut self, n: &swc_ecma_ast::MemberExpr) {
        self.hasher.write(b"MemberExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_member_prop(&mut self, n: &swc_ecma_ast::MemberProp) {
        self.hasher.write(b"MemberProp");
        match n {
            swc_ecma_ast::MemberProp::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::MemberProp::PrivateName(_) => self.hasher.write(b"PrivateName(_)"),
            swc_ecma_ast::MemberProp::Computed(_) => self.hasher.write(b"Computed(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_super_prop_expr(&mut self, n: &swc_ecma_ast::SuperPropExpr) {
        self.hasher.write(b"SuperPropExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_super_prop(&mut self, n: &swc_ecma_ast::SuperProp) {
        self.hasher.write(b"SuperProp");
        match n {
            swc_ecma_ast::SuperProp::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::SuperProp::Computed(_) => self.hasher.write(b"Computed(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_cond_expr(&mut self, n: &swc_ecma_ast::CondExpr) {
        self.hasher.write(b"CondExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_call_expr(&mut self, n: &swc_ecma_ast::CallExpr) {
        self.hasher.write(b"CallExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_new_expr(&mut self, n: &swc_ecma_ast::NewExpr) {
        self.hasher.write(b"NewExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_seq_expr(&mut self, n: &swc_ecma_ast::SeqExpr) {
        self.hasher.write(b"SeqExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_arrow_expr(&mut self, n: &swc_ecma_ast::ArrowExpr) {
        self.hasher.write(b"ArrowExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_yield_expr(&mut self, n: &swc_ecma_ast::YieldExpr) {
        self.hasher.write(b"YieldExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_meta_prop_expr(&mut self, n: &swc_ecma_ast::MetaPropExpr) {
        self.hasher.write(b"MetaPropExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_meta_prop_kind(&mut self, n: &swc_ecma_ast::MetaPropKind) {
        self.hasher.write(b"MetaPropKind");
        match n {
            swc_ecma_ast::MetaPropKind::NewTarget => self.hasher.write(b"NewTarget"),
            swc_ecma_ast::MetaPropKind::ImportMeta => self.hasher.write(b"ImportMeta"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_await_expr(&mut self, n: &swc_ecma_ast::AwaitExpr) {
        self.hasher.write(b"AwaitExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_tpl(&mut self, n: &swc_ecma_ast::Tpl) {
        self.hasher.write(b"Tpl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_tagged_tpl(&mut self, n: &swc_ecma_ast::TaggedTpl) {
        self.hasher.write(b"TaggedTpl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_tpl_element(&mut self, n: &swc_ecma_ast::TplElement) {
        self.hasher.write(b"TplElement");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_paren_expr(&mut self, n: &swc_ecma_ast::ParenExpr) {
        self.hasher.write(b"ParenExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_callee(&mut self, n: &swc_ecma_ast::Callee) {
        self.hasher.write(b"Callee");
        match n {
            swc_ecma_ast::Callee::Super(_) => self.hasher.write(b"Super(_)"),
            swc_ecma_ast::Callee::Import(_) => self.hasher.write(b"Import(_)"),
            swc_ecma_ast::Callee::Expr(_) => self.hasher.write(b"Expr(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_super(&mut self, n: &swc_ecma_ast::Super) {
        self.hasher.write(b"Super");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_import(&mut self, n: &swc_ecma_ast::Import) {
        self.hasher.write(b"Import");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_expr_or_spread(&mut self, n: &swc_ecma_ast::ExprOrSpread) {
        self.hasher.write(b"ExprOrSpread");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_block_stmt_or_expr(&mut self, n: &swc_ecma_ast::BlockStmtOrExpr) {
        self.hasher.write(b"BlockStmtOrExpr");
        match n {
            swc_ecma_ast::BlockStmtOrExpr::BlockStmt(_) => self.hasher.write(b"BlockStmt(_)"),
            swc_ecma_ast::BlockStmtOrExpr::Expr(_) => self.hasher.write(b"Expr(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_assign_target(&mut self, n: &swc_ecma_ast::AssignTarget) {
        self.hasher.write(b"AssignTarget");
        match n {
            swc_ecma_ast::AssignTarget::Simple(_) => self.hasher.write(b"Simple(_)"),
            swc_ecma_ast::AssignTarget::Pat(_) => self.hasher.write(b"Pat(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_assign_target_pat(&mut self, n: &swc_ecma_ast::AssignTargetPat) {
        self.hasher.write(b"AssignTargetPat");
        match n {
            swc_ecma_ast::AssignTargetPat::Array(_) => self.hasher.write(b"Array(_)"),
            swc_ecma_ast::AssignTargetPat::Object(_) => self.hasher.write(b"Object(_)"),
            swc_ecma_ast::AssignTargetPat::Invalid(_) => self.hasher.write(b"Invalid(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_simple_assign_target(&mut self, n: &swc_ecma_ast::SimpleAssignTarget) {
        self.hasher.write(b"SimpleAssignTarget");
        match n {
            swc_ecma_ast::SimpleAssignTarget::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::SimpleAssignTarget::Member(_) => self.hasher.write(b"Member(_)"),
            swc_ecma_ast::SimpleAssignTarget::SuperProp(_) => self.hasher.write(b"SuperProp(_)"),
            swc_ecma_ast::SimpleAssignTarget::OptChain(_) => self.hasher.write(b"OptChain(_)"),
            swc_ecma_ast::SimpleAssignTarget::Paren(_) => self.hasher.write(b"Paren(_)"),
            swc_ecma_ast::SimpleAssignTarget::TsAs(_) => self.hasher.write(b"TsAs(_)"),
            swc_ecma_ast::SimpleAssignTarget::TsSatisfies(_) => self.hasher.write(b"TsSatisfies(_)"),
            swc_ecma_ast::SimpleAssignTarget::TsNonNull(_) => self.hasher.write(b"TsNonNull(_)"),
            swc_ecma_ast::SimpleAssignTarget::TsTypeAssertion(_) => self.hasher.write(b"TsTypeAssertion(_)"),
            swc_ecma_ast::SimpleAssignTarget::TsInstantiation(_) => self.hasher.write(b"TsInstantiation(_)"),
            swc_ecma_ast::SimpleAssignTarget::Invalid(_) => self.hasher.write(b"Invalid(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_opt_chain_expr(&mut self, n: &swc_ecma_ast::OptChainExpr) {
        self.hasher.write(b"OptChainExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_opt_chain_base(&mut self, n: &swc_ecma_ast::OptChainBase) {
        self.hasher.write(b"OptChainBase");
        match n {
            swc_ecma_ast::OptChainBase::Member(_) => self.hasher.write(b"Member(_)"),
            swc_ecma_ast::OptChainBase::Call(_) => self.hasher.write(b"Call(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_opt_call(&mut self, n: &swc_ecma_ast::OptCall) {
        self.hasher.write(b"OptCall");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_function(&mut self, n: &swc_ecma_ast::Function) {
        self.hasher.write(b"Function");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_param(&mut self, n: &swc_ecma_ast::Param) {
        self.hasher.write(b"Param");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_param_or_ts_param_prop(&mut self, n: &swc_ecma_ast::ParamOrTsParamProp) {
        self.hasher.write(b"ParamOrTsParamProp");
        match n {
            swc_ecma_ast::ParamOrTsParamProp::TsParamProp(_) => self.hasher.write(b"TsParamProp(_)"),
            swc_ecma_ast::ParamOrTsParamProp::Param(_) => self.hasher.write(b"Param(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_binding_ident(&mut self, n: &swc_ecma_ast::BindingIdent) {
        self.hasher.write(b"BindingIdent");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ident(&mut self, n: &swc_ecma_ast::Ident) {
        self.hasher.write(b"Ident");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_private_name(&mut self, n: &swc_ecma_ast::PrivateName) {
        self.hasher.write(b"PrivateName");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_object(&mut self, n: &swc_ecma_ast::JSXObject) {
        self.hasher.write(b"JSXObject");
        match n {
            swc_ecma_ast::JSXObject::JSXMemberExpr(_) => self.hasher.write(b"JSXMemberExpr(_)"),
            swc_ecma_ast::JSXObject::Ident(_) => self.hasher.write(b"Ident(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_member_expr(&mut self, n: &swc_ecma_ast::JSXMemberExpr) {
        self.hasher.write(b"JSXMemberExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_namespaced_name(&mut self, n: &swc_ecma_ast::JSXNamespacedName) {
        self.hasher.write(b"JSXNamespacedName");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_empty_expr(&mut self, n: &swc_ecma_ast::JSXEmptyExpr) {
        self.hasher.write(b"JSXEmptyExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_expr_container(&mut self, n: &swc_ecma_ast::JSXExprContainer) {
        self.hasher.write(b"JSXExprContainer");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_expr(&mut self, n: &swc_ecma_ast::JSXExpr) {
        self.hasher.write(b"JSXExpr");
        match n {
            swc_ecma_ast::JSXExpr::JSXEmptyExpr(_) => self.hasher.write(b"JSXEmptyExpr(_)"),
            swc_ecma_ast::JSXExpr::Expr(_) => self.hasher.write(b"Expr(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_spread_child(&mut self, n: &swc_ecma_ast::JSXSpreadChild) {
        self.hasher.write(b"JSXSpreadChild");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_element_name(&mut self, n: &swc_ecma_ast::JSXElementName) {
        self.hasher.write(b"JSXElementName");
        match n {
            swc_ecma_ast::JSXElementName::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::JSXElementName::JSXMemberExpr(_) => self.hasher.write(b"JSXMemberExpr(_)"),
            swc_ecma_ast::JSXElementName::JSXNamespacedName(_) => self.hasher.write(b"JSXNamespacedName(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_opening_element(&mut self, n: &swc_ecma_ast::JSXOpeningElement) {
        self.hasher.write(b"JSXOpeningElement");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_attr_or_spread(&mut self, n: &swc_ecma_ast::JSXAttrOrSpread) {
        self.hasher.write(b"JSXAttrOrSpread");
        match n {
            swc_ecma_ast::JSXAttrOrSpread::JSXAttr(_) => self.hasher.write(b"JSXAttr(_)"),
            swc_ecma_ast::JSXAttrOrSpread::SpreadElement(_) => self.hasher.write(b"SpreadElement(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_closing_element(&mut self, n: &swc_ecma_ast::JSXClosingElement) {
        self.hasher.write(b"JSXClosingElement");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_attr(&mut self, n: &swc_ecma_ast::JSXAttr) {
        self.hasher.write(b"JSXAttr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_attr_name(&mut self, n: &swc_ecma_ast::JSXAttrName) {
        self.hasher.write(b"JSXAttrName");
        match n {
            swc_ecma_ast::JSXAttrName::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::JSXAttrName::JSXNamespacedName(_) => self.hasher.write(b"JSXNamespacedName(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_attr_value(&mut self, n: &swc_ecma_ast::JSXAttrValue) {
        self.hasher.write(b"JSXAttrValue");
        match n {
            swc_ecma_ast::JSXAttrValue::Lit(_) => self.hasher.write(b"Lit(_)"),
            swc_ecma_ast::JSXAttrValue::JSXExprContainer(_) => self.hasher.write(b"JSXExprContainer(_)"),
            swc_ecma_ast::JSXAttrValue::JSXElement(_) => self.hasher.write(b"JSXElement(_)"),
            swc_ecma_ast::JSXAttrValue::JSXFragment(_) => self.hasher.write(b"JSXFragment(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_text(&mut self, n: &swc_ecma_ast::JSXText) {
        self.hasher.write(b"JSXText");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_element(&mut self, n: &swc_ecma_ast::JSXElement) {
        self.hasher.write(b"JSXElement");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_element_child(&mut self, n: &swc_ecma_ast::JSXElementChild) {
        self.hasher.write(b"JSXElementChild");
        match n {
            swc_ecma_ast::JSXElementChild::JSXText(_) => self.hasher.write(b"JSXText(_)"),
            swc_ecma_ast::JSXElementChild::JSXExprContainer(_) => self.hasher.write(b"JSXExprContainer(_)"),
            swc_ecma_ast::JSXElementChild::JSXSpreadChild(_) => self.hasher.write(b"JSXSpreadChild(_)"),
            swc_ecma_ast::JSXElementChild::JSXElement(_) => self.hasher.write(b"JSXElement(_)"),
            swc_ecma_ast::JSXElementChild::JSXFragment(_) => self.hasher.write(b"JSXFragment(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_fragment(&mut self, n: &swc_ecma_ast::JSXFragment) {
        self.hasher.write(b"JSXFragment");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_opening_fragment(&mut self, n: &swc_ecma_ast::JSXOpeningFragment) {
        self.hasher.write(b"JSXOpeningFragment");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_jsx_closing_fragment(&mut self, n: &swc_ecma_ast::JSXClosingFragment) {
        self.hasher.write(b"JSXClosingFragment");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_invalid(&mut self, n: &swc_ecma_ast::Invalid) {
        self.hasher.write(b"Invalid");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_lit(&mut self, n: &swc_ecma_ast::Lit) {
        self.hasher.write(b"Lit");
        match n {
            swc_ecma_ast::Lit::Str(_) => self.hasher.write(b"Str(_)"),
            swc_ecma_ast::Lit::Bool(_) => self.hasher.write(b"Bool(_)"),
            swc_ecma_ast::Lit::Null(_) => self.hasher.write(b"Null(_)"),
            swc_ecma_ast::Lit::Num(_) => self.hasher.write(b"Num(_)"),
            swc_ecma_ast::Lit::BigInt(_) => self.hasher.write(b"BigInt(_)"),
            swc_ecma_ast::Lit::Regex(_) => self.hasher.write(b"Regex(_)"),
            swc_ecma_ast::Lit::JSXText(_) => self.hasher.write(b"JSXText(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_big_int(&mut self, n: &swc_ecma_ast::BigInt) {
        self.hasher.write(b"BigInt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_str(&mut self, n: &swc_ecma_ast::Str) {
        self.hasher.write(b"Str");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_bool(&mut self, n: &swc_ecma_ast::Bool) {
        self.hasher.write(b"Bool");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_null(&mut self, n: &swc_ecma_ast::Null) {
        self.hasher.write(b"Null");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_regex(&mut self, n: &swc_ecma_ast::Regex) {
        self.hasher.write(b"Regex");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_number(&mut self, n: &swc_ecma_ast::Number) {
        self.hasher.write(b"Number");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_program(&mut self, n: &swc_ecma_ast::Program) {
        self.hasher.write(b"Program");
        match n {
            swc_ecma_ast::Program::Module(_) => self.hasher.write(b"Module(_)"),
            swc_ecma_ast::Program::Script(_) => self.hasher.write(b"Script(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_module(&mut self, n: &swc_ecma_ast::Module) {
        self.hasher.write(b"Module");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_script(&mut self, n: &swc_ecma_ast::Script) {
        self.hasher.write(b"Script");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_module_item(&mut self, n: &swc_ecma_ast::ModuleItem) {
        self.hasher.write(b"ModuleItem");
        match n {
            swc_ecma_ast::ModuleItem::ModuleDecl(_) => self.hasher.write(b"ModuleDecl(_)"),
            swc_ecma_ast::ModuleItem::Stmt(_) => self.hasher.write(b"Stmt(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_module_decl(&mut self, n: &swc_ecma_ast::ModuleDecl) {
        self.hasher.write(b"ModuleDecl");
        match n {
            swc_ecma_ast::ModuleDecl::Import(_) => self.hasher.write(b"Import(_)"),
            swc_ecma_ast::ModuleDecl::ExportDecl(_) => self.hasher.write(b"ExportDecl(_)"),
            swc_ecma_ast::ModuleDecl::ExportNamed(_) => self.hasher.write(b"ExportNamed(_)"),
            swc_ecma_ast::ModuleDecl::ExportDefaultDecl(_) => self.hasher.write(b"ExportDefaultDecl(_)"),
            swc_ecma_ast::ModuleDecl::ExportDefaultExpr(_) => self.hasher.write(b"ExportDefaultExpr(_)"),
            swc_ecma_ast::ModuleDecl::ExportAll(_) => self.hasher.write(b"ExportAll(_)"),
            swc_ecma_ast::ModuleDecl::TsImportEquals(_) => self.hasher.write(b"TsImportEquals(_)"),
            swc_ecma_ast::ModuleDecl::TsExportAssignment(_) => self.hasher.write(b"TsExportAssignment(_)"),
            swc_ecma_ast::ModuleDecl::TsNamespaceExport(_) => self.hasher.write(b"TsNamespaceExport(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_export_default_expr(&mut self, n: &swc_ecma_ast::ExportDefaultExpr) {
        self.hasher.write(b"ExportDefaultExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_export_decl(&mut self, n: &swc_ecma_ast::ExportDecl) {
        self.hasher.write(b"ExportDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_import_decl(&mut self, n: &swc_ecma_ast::ImportDecl) {
        self.hasher.write(b"ImportDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_export_all(&mut self, n: &swc_ecma_ast::ExportAll) {
        self.hasher.write(b"ExportAll");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_named_export(&mut self, n: &swc_ecma_ast::NamedExport) {
        self.hasher.write(b"NamedExport");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_export_default_decl(&mut self, n: &swc_ecma_ast::ExportDefaultDecl) {
        self.hasher.write(b"ExportDefaultDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_default_decl(&mut self, n: &swc_ecma_ast::DefaultDecl) {
        self.hasher.write(b"DefaultDecl");
        match n {
            swc_ecma_ast::DefaultDecl::Class(_) => self.hasher.write(b"Class(_)"),
            swc_ecma_ast::DefaultDecl::Fn(_) => self.hasher.write(b"Fn(_)"),
            swc_ecma_ast::DefaultDecl::TsInterfaceDecl(_) => self.hasher.write(b"TsInterfaceDecl(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_import_specifier(&mut self, n: &swc_ecma_ast::ImportSpecifier) {
        self.hasher.write(b"ImportSpecifier");
        match n {
            swc_ecma_ast::ImportSpecifier::Named(_) => self.hasher.write(b"Named(_)"),
            swc_ecma_ast::ImportSpecifier::Default(_) => self.hasher.write(b"Default(_)"),
            swc_ecma_ast::ImportSpecifier::Namespace(_) => self.hasher.write(b"Namespace(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_import_default_specifier(&mut self, n: &swc_ecma_ast::ImportDefaultSpecifier) {
        self.hasher.write(b"ImportDefaultSpecifier");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_import_star_as_specifier(&mut self, n: &swc_ecma_ast::ImportStarAsSpecifier) {
        self.hasher.write(b"ImportStarAsSpecifier");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_import_named_specifier(&mut self, n: &swc_ecma_ast::ImportNamedSpecifier) {
        self.hasher.write(b"ImportNamedSpecifier");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_export_specifier(&mut self, n: &swc_ecma_ast::ExportSpecifier) {
        self.hasher.write(b"ExportSpecifier");
        match n {
            swc_ecma_ast::ExportSpecifier::Namespace(_) => self.hasher.write(b"Namespace(_)"),
            swc_ecma_ast::ExportSpecifier::Default(_) => self.hasher.write(b"Default(_)"),
            swc_ecma_ast::ExportSpecifier::Named(_) => self.hasher.write(b"Named(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_export_namespace_specifier(&mut self, n: &swc_ecma_ast::ExportNamespaceSpecifier) {
        self.hasher.write(b"ExportNamespaceSpecifier");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_export_default_specifier(&mut self, n: &swc_ecma_ast::ExportDefaultSpecifier) {
        self.hasher.write(b"ExportDefaultSpecifier");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_module_export_name(&mut self, n: &swc_ecma_ast::ModuleExportName) {
        self.hasher.write(b"ModuleExportName");
        match n {
            swc_ecma_ast::ModuleExportName::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::ModuleExportName::Str(_) => self.hasher.write(b"Str(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_export_named_specifier(&mut self, n: &swc_ecma_ast::ExportNamedSpecifier) {
        self.hasher.write(b"ExportNamedSpecifier");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_binary_op(&mut self, n: &swc_ecma_ast::BinaryOp) {
        self.hasher.write(b"BinaryOp");
        match n {
            swc_ecma_ast::BinaryOp::EqEq => self.hasher.write(b"EqEq"),
            swc_ecma_ast::BinaryOp::NotEq => self.hasher.write(b"NotEq"),
            swc_ecma_ast::BinaryOp::EqEqEq => self.hasher.write(b"EqEqEq"),
            swc_ecma_ast::BinaryOp::NotEqEq => self.hasher.write(b"NotEqEq"),
            swc_ecma_ast::BinaryOp::Lt => self.hasher.write(b"Lt"),
            swc_ecma_ast::BinaryOp::LtEq => self.hasher.write(b"LtEq"),
            swc_ecma_ast::BinaryOp::Gt => self.hasher.write(b"Gt"),
            swc_ecma_ast::BinaryOp::GtEq => self.hasher.write(b"GtEq"),
            swc_ecma_ast::BinaryOp::LShift => self.hasher.write(b"LShift"),
            swc_ecma_ast::BinaryOp::RShift => self.hasher.write(b"RShift"),
            swc_ecma_ast::BinaryOp::ZeroFillRShift => self.hasher.write(b"ZeroFillRShift"),
            swc_ecma_ast::BinaryOp::Add => self.hasher.write(b"Add"),
            swc_ecma_ast::BinaryOp::Sub => self.hasher.write(b"Sub"),
            swc_ecma_ast::BinaryOp::Mul => self.hasher.write(b"Mul"),
            swc_ecma_ast::BinaryOp::Div => self.hasher.write(b"Div"),
            swc_ecma_ast::BinaryOp::Mod => self.hasher.write(b"Mod"),
            swc_ecma_ast::BinaryOp::BitOr => self.hasher.write(b"BitOr"),
            swc_ecma_ast::BinaryOp::BitXor => self.hasher.write(b"BitXor"),
            swc_ecma_ast::BinaryOp::BitAnd => self.hasher.write(b"BitAnd"),
            swc_ecma_ast::BinaryOp::LogicalOr => self.hasher.write(b"LogicalOr"),
            swc_ecma_ast::BinaryOp::LogicalAnd => self.hasher.write(b"LogicalAnd"),
            swc_ecma_ast::BinaryOp::In => self.hasher.write(b"In"),
            swc_ecma_ast::BinaryOp::InstanceOf => self.hasher.write(b"InstanceOf"),
            swc_ecma_ast::BinaryOp::Exp => self.hasher.write(b"Exp"),
            swc_ecma_ast::BinaryOp::NullishCoalescing => self.hasher.write(b"NullishCoalescing"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_assign_op(&mut self, n: &swc_ecma_ast::AssignOp) {
        self.hasher.write(b"AssignOp");
        match n {
            swc_ecma_ast::AssignOp::Assign => self.hasher.write(b"Assign"),
            swc_ecma_ast::AssignOp::AddAssign => self.hasher.write(b"AddAssign"),
            swc_ecma_ast::AssignOp::SubAssign => self.hasher.write(b"SubAssign"),
            swc_ecma_ast::AssignOp::MulAssign => self.hasher.write(b"MulAssign"),
            swc_ecma_ast::AssignOp::DivAssign => self.hasher.write(b"DivAssign"),
            swc_ecma_ast::AssignOp::ModAssign => self.hasher.write(b"ModAssign"),
            swc_ecma_ast::AssignOp::LShiftAssign => self.hasher.write(b"LShiftAssign"),
            swc_ecma_ast::AssignOp::RShiftAssign => self.hasher.write(b"RShiftAssign"),
            swc_ecma_ast::AssignOp::ZeroFillRShiftAssign => self.hasher.write(b"ZeroFillRShiftAssign"),
            swc_ecma_ast::AssignOp::BitOrAssign => self.hasher.write(b"BitOrAssign"),
            swc_ecma_ast::AssignOp::BitXorAssign => self.hasher.write(b"BitXorAssign"),
            swc_ecma_ast::AssignOp::BitAndAssign => self.hasher.write(b"BitAndAssign"),
            swc_ecma_ast::AssignOp::ExpAssign => self.hasher.write(b"ExpAssign"),
            swc_ecma_ast::AssignOp::AndAssign => self.hasher.write(b"AndAssign"),
            swc_ecma_ast::AssignOp::OrAssign => self.hasher.write(b"OrAssign"),
            swc_ecma_ast::AssignOp::NullishAssign => self.hasher.write(b"NullishAssign"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_update_op(&mut self, n: &swc_ecma_ast::UpdateOp) {
        self.hasher.write(b"UpdateOp");
        match n {
            swc_ecma_ast::UpdateOp::PlusPlus => self.hasher.write(b"PlusPlus"),
            swc_ecma_ast::UpdateOp::MinusMinus => self.hasher.write(b"MinusMinus"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_unary_op(&mut self, n: &swc_ecma_ast::UnaryOp) {
        self.hasher.write(b"UnaryOp");
        match n {
            swc_ecma_ast::UnaryOp::Minus => self.hasher.write(b"Minus"),
            swc_ecma_ast::UnaryOp::Plus => self.hasher.write(b"Plus"),
            swc_ecma_ast::UnaryOp::Bang => self.hasher.write(b"Bang"),
            swc_ecma_ast::UnaryOp::Tilde => self.hasher.write(b"Tilde"),
            swc_ecma_ast::UnaryOp::TypeOf => self.hasher.write(b"TypeOf"),
            swc_ecma_ast::UnaryOp::Void => self.hasher.write(b"Void"),
            swc_ecma_ast::UnaryOp::Delete => self.hasher.write(b"Delete"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_pat(&mut self, n: &swc_ecma_ast::Pat) {
        self.hasher.write(b"Pat");
        match n {
            swc_ecma_ast::Pat::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::Pat::Array(_) => self.hasher.write(b"Array(_)"),
            swc_ecma_ast::Pat::Rest(_) => self.hasher.write(b"Rest(_)"),
            swc_ecma_ast::Pat::Object(_) => self.hasher.write(b"Object(_)"),
            swc_ecma_ast::Pat::Assign(_) => self.hasher.write(b"Assign(_)"),
            swc_ecma_ast::Pat::Invalid(_) => self.hasher.write(b"Invalid(_)"),
            swc_ecma_ast::Pat::Expr(_) => self.hasher.write(b"Expr(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_array_pat(&mut self, n: &swc_ecma_ast::ArrayPat) {
        self.hasher.write(b"ArrayPat");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_object_pat(&mut self, n: &swc_ecma_ast::ObjectPat) {
        self.hasher.write(b"ObjectPat");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_assign_pat(&mut self, n: &swc_ecma_ast::AssignPat) {
        self.hasher.write(b"AssignPat");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_rest_pat(&mut self, n: &swc_ecma_ast::RestPat) {
        self.hasher.write(b"RestPat");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_object_pat_prop(&mut self, n: &swc_ecma_ast::ObjectPatProp) {
        self.hasher.write(b"ObjectPatProp");
        match n {
            swc_ecma_ast::ObjectPatProp::KeyValue(_) => self.hasher.write(b"KeyValue(_)"),
            swc_ecma_ast::ObjectPatProp::Assign(_) => self.hasher.write(b"Assign(_)"),
            swc_ecma_ast::ObjectPatProp::Rest(_) => self.hasher.write(b"Rest(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_key_value_pat_prop(&mut self, n: &swc_ecma_ast::KeyValuePatProp) {
        self.hasher.write(b"KeyValuePatProp");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_assign_pat_prop(&mut self, n: &swc_ecma_ast::AssignPatProp) {
        self.hasher.write(b"AssignPatProp");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_prop(&mut self, n: &swc_ecma_ast::Prop) {
        self.hasher.write(b"Prop");
        match n {
            swc_ecma_ast::Prop::Shorthand(_) => self.hasher.write(b"Shorthand(_)"),
            swc_ecma_ast::Prop::KeyValue(_) => self.hasher.write(b"KeyValue(_)"),
            swc_ecma_ast::Prop::Assign(_) => self.hasher.write(b"Assign(_)"),
            swc_ecma_ast::Prop::Getter(_) => self.hasher.write(b"Getter(_)"),
            swc_ecma_ast::Prop::Setter(_) => self.hasher.write(b"Setter(_)"),
            swc_ecma_ast::Prop::Method(_) => self.hasher.write(b"Method(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_key_value_prop(&mut self, n: &swc_ecma_ast::KeyValueProp) {
        self.hasher.write(b"KeyValueProp");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_assign_prop(&mut self, n: &swc_ecma_ast::AssignProp) {
        self.hasher.write(b"AssignProp");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_getter_prop(&mut self, n: &swc_ecma_ast::GetterProp) {
        self.hasher.write(b"GetterProp");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_setter_prop(&mut self, n: &swc_ecma_ast::SetterProp) {
        self.hasher.write(b"SetterProp");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_method_prop(&mut self, n: &swc_ecma_ast::MethodProp) {
        self.hasher.write(b"MethodProp");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_prop_name(&mut self, n: &swc_ecma_ast::PropName) {
        self.hasher.write(b"PropName");
        match n {
            swc_ecma_ast::PropName::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::PropName::Str(_) => self.hasher.write(b"Str(_)"),
            swc_ecma_ast::PropName::Num(_) => self.hasher.write(b"Num(_)"),
            swc_ecma_ast::PropName::BigInt(_) => self.hasher.write(b"BigInt(_)"),
            swc_ecma_ast::PropName::Computed(_) => self.hasher.write(b"Computed(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_computed_prop_name(&mut self, n: &swc_ecma_ast::ComputedPropName) {
        self.hasher.write(b"ComputedPropName");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_block_stmt(&mut self, n: &swc_ecma_ast::BlockStmt) {
        self.hasher.write(b"BlockStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_stmt(&mut self, n: &swc_ecma_ast::Stmt) {
        self.hasher.write(b"Stmt");
        match n {
            swc_ecma_ast::Stmt::Block(_) => self.hasher.write(b"Block(_)"),
            swc_ecma_ast::Stmt::Empty(_) => self.hasher.write(b"Empty(_)"),
            swc_ecma_ast::Stmt::Debugger(_) => self.hasher.write(b"Debugger(_)"),
            swc_ecma_ast::Stmt::With(_) => self.hasher.write(b"With(_)"),
            swc_ecma_ast::Stmt::Return(_) => self.hasher.write(b"Return(_)"),
            swc_ecma_ast::Stmt::Labeled(_) => self.hasher.write(b"Labeled(_)"),
            swc_ecma_ast::Stmt::Break(_) => self.hasher.write(b"Break(_)"),
            swc_ecma_ast::Stmt::Continue(_) => self.hasher.write(b"Continue(_)"),
            swc_ecma_ast::Stmt::If(_) => self.hasher.write(b"If(_)"),
            swc_ecma_ast::Stmt::Switch(_) => self.hasher.write(b"Switch(_)"),
            swc_ecma_ast::Stmt::Throw(_) => self.hasher.write(b"Throw(_)"),
            swc_ecma_ast::Stmt::Try(_) => self.hasher.write(b"Try(_)"),
            swc_ecma_ast::Stmt::While(_) => self.hasher.write(b"While(_)"),
            swc_ecma_ast::Stmt::DoWhile(_) => self.hasher.write(b"DoWhile(_)"),
            swc_ecma_ast::Stmt::For(_) => self.hasher.write(b"For(_)"),
            swc_ecma_ast::Stmt::ForIn(_) => self.hasher.write(b"ForIn(_)"),
            swc_ecma_ast::Stmt::ForOf(_) => self.hasher.write(b"ForOf(_)"),
            swc_ecma_ast::Stmt::Decl(_) => self.hasher.write(b"Decl(_)"),
            swc_ecma_ast::Stmt::Expr(_) => self.hasher.write(b"Expr(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_expr_stmt(&mut self, n: &swc_ecma_ast::ExprStmt) {
        self.hasher.write(b"ExprStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_empty_stmt(&mut self, n: &swc_ecma_ast::EmptyStmt) {
        self.hasher.write(b"EmptyStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_debugger_stmt(&mut self, n: &swc_ecma_ast::DebuggerStmt) {
        self.hasher.write(b"DebuggerStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_with_stmt(&mut self, n: &swc_ecma_ast::WithStmt) {
        self.hasher.write(b"WithStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_return_stmt(&mut self, n: &swc_ecma_ast::ReturnStmt) {
        self.hasher.write(b"ReturnStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_labeled_stmt(&mut self, n: &swc_ecma_ast::LabeledStmt) {
        self.hasher.write(b"LabeledStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_break_stmt(&mut self, n: &swc_ecma_ast::BreakStmt) {
        self.hasher.write(b"BreakStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_continue_stmt(&mut self, n: &swc_ecma_ast::ContinueStmt) {
        self.hasher.write(b"ContinueStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_if_stmt(&mut self, n: &swc_ecma_ast::IfStmt) {
        self.hasher.write(b"IfStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_switch_stmt(&mut self, n: &swc_ecma_ast::SwitchStmt) {
        self.hasher.write(b"SwitchStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_throw_stmt(&mut self, n: &swc_ecma_ast::ThrowStmt) {
        self.hasher.write(b"ThrowStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_try_stmt(&mut self, n: &swc_ecma_ast::TryStmt) {
        self.hasher.write(b"TryStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_while_stmt(&mut self, n: &swc_ecma_ast::WhileStmt) {
        self.hasher.write(b"WhileStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_do_while_stmt(&mut self, n: &swc_ecma_ast::DoWhileStmt) {
        self.hasher.write(b"DoWhileStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_for_stmt(&mut self, n: &swc_ecma_ast::ForStmt) {
        self.hasher.write(b"ForStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_for_in_stmt(&mut self, n: &swc_ecma_ast::ForInStmt) {
        self.hasher.write(b"ForInStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_for_of_stmt(&mut self, n: &swc_ecma_ast::ForOfStmt) {
        self.hasher.write(b"ForOfStmt");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_switch_case(&mut self, n: &swc_ecma_ast::SwitchCase) {
        self.hasher.write(b"SwitchCase");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_catch_clause(&mut self, n: &swc_ecma_ast::CatchClause) {
        self.hasher.write(b"CatchClause");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_for_head(&mut self, n: &swc_ecma_ast::ForHead) {
        self.hasher.write(b"ForHead");
        match n {
            swc_ecma_ast::ForHead::VarDecl(_) => self.hasher.write(b"VarDecl(_)"),
            swc_ecma_ast::ForHead::UsingDecl(_) => self.hasher.write(b"UsingDecl(_)"),
            swc_ecma_ast::ForHead::Pat(_) => self.hasher.write(b"Pat(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_var_decl_or_expr(&mut self, n: &swc_ecma_ast::VarDeclOrExpr) {
        self.hasher.write(b"VarDeclOrExpr");
        match n {
            swc_ecma_ast::VarDeclOrExpr::VarDecl(_) => self.hasher.write(b"VarDecl(_)"),
            swc_ecma_ast::VarDeclOrExpr::Expr(_) => self.hasher.write(b"Expr(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_ann(&mut self, n: &swc_ecma_ast::TsTypeAnn) {
        self.hasher.write(b"TsTypeAnn");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_param_decl(&mut self, n: &swc_ecma_ast::TsTypeParamDecl) {
        self.hasher.write(b"TsTypeParamDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_param(&mut self, n: &swc_ecma_ast::TsTypeParam) {
        self.hasher.write(b"TsTypeParam");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_param_instantiation(&mut self, n: &swc_ecma_ast::TsTypeParamInstantiation) {
        self.hasher.write(b"TsTypeParamInstantiation");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_param_prop(&mut self, n: &swc_ecma_ast::TsParamProp) {
        self.hasher.write(b"TsParamProp");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_param_prop_param(&mut self, n: &swc_ecma_ast::TsParamPropParam) {
        self.hasher.write(b"TsParamPropParam");
        match n {
            swc_ecma_ast::TsParamPropParam::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::TsParamPropParam::Assign(_) => self.hasher.write(b"Assign(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_qualified_name(&mut self, n: &swc_ecma_ast::TsQualifiedName) {
        self.hasher.write(b"TsQualifiedName");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_entity_name(&mut self, n: &swc_ecma_ast::TsEntityName) {
        self.hasher.write(b"TsEntityName");
        match n {
            swc_ecma_ast::TsEntityName::TsQualifiedName(_) => self.hasher.write(b"TsQualifiedName(_)"),
            swc_ecma_ast::TsEntityName::Ident(_) => self.hasher.write(b"Ident(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_element(&mut self, n: &swc_ecma_ast::TsTypeElement) {
        self.hasher.write(b"TsTypeElement");
        match n {
            swc_ecma_ast::TsTypeElement::TsCallSignatureDecl(_) => self.hasher.write(b"TsCallSignatureDecl(_)"),
            swc_ecma_ast::TsTypeElement::TsConstructSignatureDecl(_) => self.hasher.write(b"TsConstructSignatureDecl(_)"),
            swc_ecma_ast::TsTypeElement::TsPropertySignature(_) => self.hasher.write(b"TsPropertySignature(_)"),
            swc_ecma_ast::TsTypeElement::TsGetterSignature(_) => self.hasher.write(b"TsGetterSignature(_)"),
            swc_ecma_ast::TsTypeElement::TsSetterSignature(_) => self.hasher.write(b"TsSetterSignature(_)"),
            swc_ecma_ast::TsTypeElement::TsMethodSignature(_) => self.hasher.write(b"TsMethodSignature(_)"),
            swc_ecma_ast::TsTypeElement::TsIndexSignature(_) => self.hasher.write(b"TsIndexSignature(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_call_signature_decl(&mut self, n: &swc_ecma_ast::TsCallSignatureDecl) {
        self.hasher.write(b"TsCallSignatureDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_construct_signature_decl(&mut self, n: &swc_ecma_ast::TsConstructSignatureDecl) {
        self.hasher.write(b"TsConstructSignatureDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_property_signature(&mut self, n: &swc_ecma_ast::TsPropertySignature) {
        self.hasher.write(b"TsPropertySignature");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_getter_signature(&mut self, n: &swc_ecma_ast::TsGetterSignature) {
        self.hasher.write(b"TsGetterSignature");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_setter_signature(&mut self, n: &swc_ecma_ast::TsSetterSignature) {
        self.hasher.write(b"TsSetterSignature");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_method_signature(&mut self, n: &swc_ecma_ast::TsMethodSignature) {
        self.hasher.write(b"TsMethodSignature");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_index_signature(&mut self, n: &swc_ecma_ast::TsIndexSignature) {
        self.hasher.write(b"TsIndexSignature");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type(&mut self, n: &swc_ecma_ast::TsType) {
        self.hasher.write(b"TsType");
        match n {
            swc_ecma_ast::TsType::TsKeywordType(_) => self.hasher.write(b"TsKeywordType(_)"),
            swc_ecma_ast::TsType::TsThisType(_) => self.hasher.write(b"TsThisType(_)"),
            swc_ecma_ast::TsType::TsFnOrConstructorType(_) => self.hasher.write(b"TsFnOrConstructorType(_)"),
            swc_ecma_ast::TsType::TsTypeRef(_) => self.hasher.write(b"TsTypeRef(_)"),
            swc_ecma_ast::TsType::TsTypeQuery(_) => self.hasher.write(b"TsTypeQuery(_)"),
            swc_ecma_ast::TsType::TsTypeLit(_) => self.hasher.write(b"TsTypeLit(_)"),
            swc_ecma_ast::TsType::TsArrayType(_) => self.hasher.write(b"TsArrayType(_)"),
            swc_ecma_ast::TsType::TsTupleType(_) => self.hasher.write(b"TsTupleType(_)"),
            swc_ecma_ast::TsType::TsOptionalType(_) => self.hasher.write(b"TsOptionalType(_)"),
            swc_ecma_ast::TsType::TsRestType(_) => self.hasher.write(b"TsRestType(_)"),
            swc_ecma_ast::TsType::TsUnionOrIntersectionType(_) => self.hasher.write(b"TsUnionOrIntersectionType(_)"),
            swc_ecma_ast::TsType::TsConditionalType(_) => self.hasher.write(b"TsConditionalType(_)"),
            swc_ecma_ast::TsType::TsInferType(_) => self.hasher.write(b"TsInferType(_)"),
            swc_ecma_ast::TsType::TsParenthesizedType(_) => self.hasher.write(b"TsParenthesizedType(_)"),
            swc_ecma_ast::TsType::TsTypeOperator(_) => self.hasher.write(b"TsTypeOperator(_)"),
            swc_ecma_ast::TsType::TsIndexedAccessType(_) => self.hasher.write(b"TsIndexedAccessType(_)"),
            swc_ecma_ast::TsType::TsMappedType(_) => self.hasher.write(b"TsMappedType(_)"),
            swc_ecma_ast::TsType::TsLitType(_) => self.hasher.write(b"TsLitType(_)"),
            swc_ecma_ast::TsType::TsTypePredicate(_) => self.hasher.write(b"TsTypePredicate(_)"),
            swc_ecma_ast::TsType::TsImportType(_) => self.hasher.write(b"TsImportType(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_fn_or_constructor_type(&mut self, n: &swc_ecma_ast::TsFnOrConstructorType) {
        self.hasher.write(b"TsFnOrConstructorType");
        match n {
            swc_ecma_ast::TsFnOrConstructorType::TsFnType(_) => self.hasher.write(b"TsFnType(_)"),
            swc_ecma_ast::TsFnOrConstructorType::TsConstructorType(_) => self.hasher.write(b"TsConstructorType(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_keyword_type(&mut self, n: &swc_ecma_ast::TsKeywordType) {
        self.hasher.write(b"TsKeywordType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_keyword_type_kind(&mut self, n: &swc_ecma_ast::TsKeywordTypeKind) {
        self.hasher.write(b"TsKeywordTypeKind");
        match n {
            swc_ecma_ast::TsKeywordTypeKind::TsAnyKeyword => self.hasher.write(b"TsAnyKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsUnknownKeyword => self.hasher.write(b"TsUnknownKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsNumberKeyword => self.hasher.write(b"TsNumberKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsObjectKeyword => self.hasher.write(b"TsObjectKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsBooleanKeyword => self.hasher.write(b"TsBooleanKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsBigIntKeyword => self.hasher.write(b"TsBigIntKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsStringKeyword => self.hasher.write(b"TsStringKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsSymbolKeyword => self.hasher.write(b"TsSymbolKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsVoidKeyword => self.hasher.write(b"TsVoidKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsUndefinedKeyword => self.hasher.write(b"TsUndefinedKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsNullKeyword => self.hasher.write(b"TsNullKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsNeverKeyword => self.hasher.write(b"TsNeverKeyword"),
            swc_ecma_ast::TsKeywordTypeKind::TsIntrinsicKeyword => self.hasher.write(b"TsIntrinsicKeyword"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_this_type(&mut self, n: &swc_ecma_ast::TsThisType) {
        self.hasher.write(b"TsThisType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_fn_param(&mut self, n: &swc_ecma_ast::TsFnParam) {
        self.hasher.write(b"TsFnParam");
        match n {
            swc_ecma_ast::TsFnParam::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::TsFnParam::Array(_) => self.hasher.write(b"Array(_)"),
            swc_ecma_ast::TsFnParam::Rest(_) => self.hasher.write(b"Rest(_)"),
            swc_ecma_ast::TsFnParam::Object(_) => self.hasher.write(b"Object(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_fn_type(&mut self, n: &swc_ecma_ast::TsFnType) {
        self.hasher.write(b"TsFnType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_constructor_type(&mut self, n: &swc_ecma_ast::TsConstructorType) {
        self.hasher.write(b"TsConstructorType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_ref(&mut self, n: &swc_ecma_ast::TsTypeRef) {
        self.hasher.write(b"TsTypeRef");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_predicate(&mut self, n: &swc_ecma_ast::TsTypePredicate) {
        self.hasher.write(b"TsTypePredicate");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_this_type_or_ident(&mut self, n: &swc_ecma_ast::TsThisTypeOrIdent) {
        self.hasher.write(b"TsThisTypeOrIdent");
        match n {
            swc_ecma_ast::TsThisTypeOrIdent::TsThisType(_) => self.hasher.write(b"TsThisType(_)"),
            swc_ecma_ast::TsThisTypeOrIdent::Ident(_) => self.hasher.write(b"Ident(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_query(&mut self, n: &swc_ecma_ast::TsTypeQuery) {
        self.hasher.write(b"TsTypeQuery");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_query_expr(&mut self, n: &swc_ecma_ast::TsTypeQueryExpr) {
        self.hasher.write(b"TsTypeQueryExpr");
        match n {
            swc_ecma_ast::TsTypeQueryExpr::TsEntityName(_) => self.hasher.write(b"TsEntityName(_)"),
            swc_ecma_ast::TsTypeQueryExpr::Import(_) => self.hasher.write(b"Import(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_import_type(&mut self, n: &swc_ecma_ast::TsImportType) {
        self.hasher.write(b"TsImportType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_lit(&mut self, n: &swc_ecma_ast::TsTypeLit) {
        self.hasher.write(b"TsTypeLit");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_array_type(&mut self, n: &swc_ecma_ast::TsArrayType) {
        self.hasher.write(b"TsArrayType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_tuple_type(&mut self, n: &swc_ecma_ast::TsTupleType) {
        self.hasher.write(b"TsTupleType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_tuple_element(&mut self, n: &swc_ecma_ast::TsTupleElement) {
        self.hasher.write(b"TsTupleElement");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_optional_type(&mut self, n: &swc_ecma_ast::TsOptionalType) {
        self.hasher.write(b"TsOptionalType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_rest_type(&mut self, n: &swc_ecma_ast::TsRestType) {
        self.hasher.write(b"TsRestType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_union_or_intersection_type(&mut self, n: &swc_ecma_ast::TsUnionOrIntersectionType) {
        self.hasher.write(b"TsUnionOrIntersectionType");
        match n {
            swc_ecma_ast::TsUnionOrIntersectionType::TsUnionType(_) => self.hasher.write(b"TsUnionType(_)"),
            swc_ecma_ast::TsUnionOrIntersectionType::TsIntersectionType(_) => self.hasher.write(b"TsIntersectionType(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_union_type(&mut self, n: &swc_ecma_ast::TsUnionType) {
        self.hasher.write(b"TsUnionType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_intersection_type(&mut self, n: &swc_ecma_ast::TsIntersectionType) {
        self.hasher.write(b"TsIntersectionType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_conditional_type(&mut self, n: &swc_ecma_ast::TsConditionalType) {
        self.hasher.write(b"TsConditionalType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_infer_type(&mut self, n: &swc_ecma_ast::TsInferType) {
        self.hasher.write(b"TsInferType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_parenthesized_type(&mut self, n: &swc_ecma_ast::TsParenthesizedType) {
        self.hasher.write(b"TsParenthesizedType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_operator(&mut self, n: &swc_ecma_ast::TsTypeOperator) {
        self.hasher.write(b"TsTypeOperator");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_operator_op(&mut self, n: &swc_ecma_ast::TsTypeOperatorOp) {
        self.hasher.write(b"TsTypeOperatorOp");
        match n {
            swc_ecma_ast::TsTypeOperatorOp::KeyOf => self.hasher.write(b"KeyOf"),
            swc_ecma_ast::TsTypeOperatorOp::Unique => self.hasher.write(b"Unique"),
            swc_ecma_ast::TsTypeOperatorOp::ReadOnly => self.hasher.write(b"ReadOnly"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_indexed_access_type(&mut self, n: &swc_ecma_ast::TsIndexedAccessType) {
        self.hasher.write(b"TsIndexedAccessType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_true_plus_minus(&mut self, n: &swc_ecma_ast::TruePlusMinus) {
        self.hasher.write(b"TruePlusMinus");
        match n {
            swc_ecma_ast::TruePlusMinus::True => self.hasher.write(b"True"),
            swc_ecma_ast::TruePlusMinus::Plus => self.hasher.write(b"Plus"),
            swc_ecma_ast::TruePlusMinus::Minus => self.hasher.write(b"Minus"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_mapped_type(&mut self, n: &swc_ecma_ast::TsMappedType) {
        self.hasher.write(b"TsMappedType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_lit_type(&mut self, n: &swc_ecma_ast::TsLitType) {
        self.hasher.write(b"TsLitType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_lit(&mut self, n: &swc_ecma_ast::TsLit) {
        self.hasher.write(b"TsLit");
        match n {
            swc_ecma_ast::TsLit::BigInt(_) => self.hasher.write(b"BigInt(_)"),
            swc_ecma_ast::TsLit::Number(_) => self.hasher.write(b"Number(_)"),
            swc_ecma_ast::TsLit::Str(_) => self.hasher.write(b"Str(_)"),
            swc_ecma_ast::TsLit::Bool(_) => self.hasher.write(b"Bool(_)"),
            swc_ecma_ast::TsLit::Tpl(_) => self.hasher.write(b"Tpl(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_tpl_lit_type(&mut self, n: &swc_ecma_ast::TsTplLitType) {
        self.hasher.write(b"TsTplLitType");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_interface_decl(&mut self, n: &swc_ecma_ast::TsInterfaceDecl) {
        self.hasher.write(b"TsInterfaceDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_interface_body(&mut self, n: &swc_ecma_ast::TsInterfaceBody) {
        self.hasher.write(b"TsInterfaceBody");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_expr_with_type_args(&mut self, n: &swc_ecma_ast::TsExprWithTypeArgs) {
        self.hasher.write(b"TsExprWithTypeArgs");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_alias_decl(&mut self, n: &swc_ecma_ast::TsTypeAliasDecl) {
        self.hasher.write(b"TsTypeAliasDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_enum_decl(&mut self, n: &swc_ecma_ast::TsEnumDecl) {
        self.hasher.write(b"TsEnumDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_enum_member(&mut self, n: &swc_ecma_ast::TsEnumMember) {
        self.hasher.write(b"TsEnumMember");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_enum_member_id(&mut self, n: &swc_ecma_ast::TsEnumMemberId) {
        self.hasher.write(b"TsEnumMemberId");
        match n {
            swc_ecma_ast::TsEnumMemberId::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::TsEnumMemberId::Str(_) => self.hasher.write(b"Str(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_module_decl(&mut self, n: &swc_ecma_ast::TsModuleDecl) {
        self.hasher.write(b"TsModuleDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_namespace_body(&mut self, n: &swc_ecma_ast::TsNamespaceBody) {
        self.hasher.write(b"TsNamespaceBody");
        match n {
            swc_ecma_ast::TsNamespaceBody::TsModuleBlock(_) => self.hasher.write(b"TsModuleBlock(_)"),
            swc_ecma_ast::TsNamespaceBody::TsNamespaceDecl(_) => self.hasher.write(b"TsNamespaceDecl(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_module_block(&mut self, n: &swc_ecma_ast::TsModuleBlock) {
        self.hasher.write(b"TsModuleBlock");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_namespace_decl(&mut self, n: &swc_ecma_ast::TsNamespaceDecl) {
        self.hasher.write(b"TsNamespaceDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_module_name(&mut self, n: &swc_ecma_ast::TsModuleName) {
        self.hasher.write(b"TsModuleName");
        match n {
            swc_ecma_ast::TsModuleName::Ident(_) => self.hasher.write(b"Ident(_)"),
            swc_ecma_ast::TsModuleName::Str(_) => self.hasher.write(b"Str(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_import_equals_decl(&mut self, n: &swc_ecma_ast::TsImportEqualsDecl) {
        self.hasher.write(b"TsImportEqualsDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_module_ref(&mut self, n: &swc_ecma_ast::TsModuleRef) {
        self.hasher.write(b"TsModuleRef");
        match n {
            swc_ecma_ast::TsModuleRef::TsEntityName(_) => self.hasher.write(b"TsEntityName(_)"),
            swc_ecma_ast::TsModuleRef::TsExternalModuleRef(_) => self.hasher.write(b"TsExternalModuleRef(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_external_module_ref(&mut self, n: &swc_ecma_ast::TsExternalModuleRef) {
        self.hasher.write(b"TsExternalModuleRef");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_export_assignment(&mut self, n: &swc_ecma_ast::TsExportAssignment) {
        self.hasher.write(b"TsExportAssignment");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_namespace_export_decl(&mut self, n: &swc_ecma_ast::TsNamespaceExportDecl) {
        self.hasher.write(b"TsNamespaceExportDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_as_expr(&mut self, n: &swc_ecma_ast::TsAsExpr) {
        self.hasher.write(b"TsAsExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_type_assertion(&mut self, n: &swc_ecma_ast::TsTypeAssertion) {
        self.hasher.write(b"TsTypeAssertion");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_non_null_expr(&mut self, n: &swc_ecma_ast::TsNonNullExpr) {
        self.hasher.write(b"TsNonNullExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_accessibility(&mut self, n: &swc_ecma_ast::Accessibility) {
        self.hasher.write(b"Accessibility");
        match n {
            swc_ecma_ast::Accessibility::Public => self.hasher.write(b"Public"),
            swc_ecma_ast::Accessibility::Protected => self.hasher.write(b"Protected"),
            swc_ecma_ast::Accessibility::Private => self.hasher.write(b"Private"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_const_assertion(&mut self, n: &swc_ecma_ast::TsConstAssertion) {
        self.hasher.write(b"TsConstAssertion");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_instantiation(&mut self, n: &swc_ecma_ast::TsInstantiation) {
        self.hasher.write(b"TsInstantiation");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_ts_satisfies_expr(&mut self, n: &swc_ecma_ast::TsSatisfiesExpr) {
        self.hasher.write(b"TsSatisfiesExpr");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_reserved_unused(&mut self, n: &swc_ecma_ast::ReservedUnused) {
        self.hasher.write(b"ReservedUnused");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_auto_accessor(&mut self, n: &swc_ecma_ast::AutoAccessor) {
        self.hasher.write(b"AutoAccessor");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_key(&mut self, n: &swc_ecma_ast::Key) {
        self.hasher.write(b"Key");
        match n {
            swc_ecma_ast::Key::Private(_) => self.hasher.write(b"Private(_)"),
            swc_ecma_ast::Key::Public(_) => self.hasher.write(b"Public(_)"),
        }
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

    fn visit_using_decl(&mut self, n: &swc_ecma_ast::UsingDecl) {
        self.hasher.write(b"UsingDecl");
        swc_ecma_visit::VisitWith::visit_children_with(n, self);
    }

}
