use super::ReleaseArtifact;
use resast::prelude::*;

/// This prints out an overview of whatever JS you feed it. This is used to
/// (manually) figure out the parse tree needed for the WASM search algorithms.
/// The gist of it is that you can run this on a new Solidity Release Artifact,
/// and grep for "too long to print". If you look at what it outputs, you'll
/// see something like...
///
/// ```text
/// 0.8.9 ProgramPart::Decl Decl::Var(kind=Var) Pat::Ident(name=wasmBinaryFile) Expr::Lit Lit::String StringLit::Double(len=26110161, value=<too long to print>)
/// ```
///
/// or...
///
/// ```text
/// 0.8.10 ProgramPart::Stmt Stmt::Expr Expr::Assign(operator=Equal) Expr::Lit Lit::String StringLit::Double(len=26976533, value=<too long to print>)
/// ```
///
/// or...
///
/// ```text
/// 0.8.12 ProgramPart::Stmt Stmt::Expr Expr::Assign(operator=Equal) Expr::Call(argument) Expr::Lit Lit::String StringLit::Double(len=8336211, value=<too long to print>)
/// ```
///
/// And that should give you a rough map of where the WASM is, inside the JS.
///
/// This is useful because Solidity's release artifacts are compiled from
/// C++ sources, with no particular promises with regards to toolchain
/// versioning or the internal structure of the artifacts.
///
/// This means that say... upgrading emscripten may produce different release
/// artifacts from the same input sources. This is a good thing for Solidity,
/// if somewhat inconvenient for people relying on something like that.
///
/// So this function produces a map to help you figure out what's going on.
///
/// For a good example of this, compare the difference between Solidity
/// v0.8.9 and v0.8.10. v0.8.12 changed again, quite significantly.
///
/// It will likely happen again in the future, and this is for those times.
///
pub(crate) fn overview(artifact: &ReleaseArtifact) -> Result<(), ressa::Error> {
  let version = artifact.metadata().version();
  let prefix = format!("{}.{}.{}", &version.major, &version.minor, &version.patch);

  let program = ressa::Parser::new(artifact.program())?;
  for part in program {
    if let Ok(part) = part {
      describe_program_part(part, &prefix)
    } else {
      println!("ERROR: {}", part.unwrap_err());
    }
  }
  Ok(())
}

fn describe_program_part(part: ProgramPart, prefix: &str) {
  match part {
    ProgramPart::Dir(_part) => println!("{prefix} ProgramPart::Dir"),
    ProgramPart::Decl(decl) => {
      let prefix = format!("{prefix} ProgramPart::Decl");
      match decl {
        Decl::Class(_decl) => println!("{prefix} Decl::Class"),
        Decl::Export(_decl) => println!("{prefix} Decl::Export"),
        Decl::Func(func) => describe_decl_func(func, &prefix),
        Decl::Import(_decl) => println!("{prefix} Decl::Import"),
        Decl::Var(kind, vars) => {
          describe_decl_var(kind, vars, &prefix);
        }
      }
    }
    ProgramPart::Stmt(stmt) => {
      let prefix = format!("{prefix} ProgramPart::Stmt");
      match stmt {
        Stmt::Block(_stmt) => println!("{prefix} Stmt::Block"),
        Stmt::Break(_stmt) => println!("{prefix} Stmt::Break"),
        Stmt::Continue(_stmt) => println!("{prefix} Stmt::Continue"),
        Stmt::Debugger => println!("{prefix} Stmt::Debugger"),
        Stmt::DoWhile(_stmt) => println!("{prefix} Stmt::DoWhile"),
        Stmt::Empty => println!("{prefix} Stmt::Empty"),
        Stmt::Expr(expr) => {
          let prefix = format!("{prefix} Stmt::Expr");
          describe_expr(expr, &prefix)
        }
        Stmt::For(_stmt) => println!("{prefix} Stmt::For"),
        Stmt::ForIn(_stmt) => println!("{prefix} Stmt::ForIn"),
        Stmt::ForOf(_stmt) => println!("{prefix} Stmt::ForOf"),
        Stmt::If(_stmt) => println!("{prefix} Stmt::If"),
        Stmt::Labeled(_stmt) => println!("{prefix} Stmt::Labeled"),
        Stmt::Return(_stmt) => println!("{prefix} Stmt::Return"),
        Stmt::Switch(_stmt) => println!("{prefix} Stmt::Switch"),
        Stmt::Throw(_stmt) => println!("{prefix} Stmt::Throw"),
        Stmt::Try(_stmt) => println!("{prefix} Stmt::Try"),
        Stmt::Var(_stmt) => println!("{prefix} Stmt::Var"),
        Stmt::While(_stmt) => println!("{prefix} Stmt::While"),
        Stmt::With(_stmt) => println!("{prefix} Stmt::With"),
      }
    }
  }
}

fn describe_decl_func(func: Func, prefix: &str) {
  match func.id {
    None => println!("{prefix} Decl::Func(anonymous)"),
    Some(ident) => println!("{prefix} Decl::Func(name={})", &ident.name),
  }
}

fn describe_decl_var(kind: VarKind, vars: Vec<VarDecl>, prefix: &str) {
  let prefix = format!("{prefix} Decl::Var(kind={:?})", kind);
  for var in vars {
    let prefix = describe_pat(var.id, &prefix);
    if let Some(expr) = var.init {
      describe_expr(expr, &prefix);
    } else {
      println!("{prefix}");
    }
  }
}

fn describe_pat(pat: Pat, prefix: &str) -> String {
  match pat {
    Pat::Array(array) => format!("{prefix} Pat::Array(len={})", array.len()),
    Pat::Assign(_assign) => format!("{prefix} Pat::Assign"),
    Pat::Ident(ident) => format!("{prefix} Pat::Ident(name={})", &ident.name),
    Pat::Obj(_obj) => format!("{prefix} Pat::Obj"),
    Pat::RestElement(pat) => {
      let prefix = format!("{prefix} Pat::RestElement");
      describe_pat(*pat, &prefix)
    }
  }
}

fn describe_func(func: Func, prefix: &str) {
  let name = match func.id {
    None => "anonymous".to_string(),
    Some(ident) => format!("name={}", &ident.name),
  };

  let prefix = format!("{prefix} Func({}, generator={})", name, func.generator);
  for param in func.params {
    match param {
      FuncArg::Expr(expr) => describe_expr(expr, &prefix),
      FuncArg::Pat(pat) => println!("{}", describe_pat(pat, &prefix)),
    }
  }
  for part in func.body.0 {
    describe_program_part(part, &prefix);
  }
}

fn describe_expr(expr: Expr, prefix: &str) {
  match expr {
    Expr::Array(_expr) => println!("{prefix} Expr::array"),
    Expr::ArrowFunc(_expr) => println!("{prefix} Expr::ArrowFunc"),
    Expr::ArrowParamPlaceHolder(_expr, _bool) => {
      println!("{prefix} Expr::ArrowParamPlaceHolder")
    }
    Expr::Assign(assign_expr) => {
      let prefix = format!(
        "{} Expr::Assign(operator={:?})",
        &prefix, assign_expr.operator
      );
      match assign_expr.left {
        AssignLeft::Pat(pat) => {
          let prefix = format!("{prefix} AssignLeft::Pat");
          let pat = describe_pat(pat, &prefix);
          println!("{pat}");
        }
        AssignLeft::Expr(expr) => {
          let prefix = format!("{prefix} AssignLeft::Expr");
          describe_expr(*expr, &prefix);
        }
      };
      describe_expr(*assign_expr.right, &prefix);
    }
    Expr::Await(_expr) => println!("{prefix} Expr::Await"),
    Expr::Binary(binary_expr) => {
      let prefix = format!(
        "{prefix} Expr::Binary(operator={:?}) Left",
        binary_expr.operator
      );
      describe_expr(*binary_expr.left, &prefix);
      let prefix = format!(
        "{prefix} Expr::Binary(operator={:?}) Right",
        binary_expr.operator
      );
      describe_expr(*binary_expr.right, &prefix);
    }
    Expr::Call(call_expr) => {
      {
        for argument in call_expr.arguments {
          let prefix = format!("{prefix} Expr::Call(argument)");
          describe_expr(argument, &prefix);
        }
      }

      {
        let prefix = format!("{prefix} Expr::Call(Callee)");
        describe_expr(*call_expr.callee, &prefix)
      }
    }
    Expr::Class(_class) => println!("{prefix} Expr::Class"),
    Expr::Conditional(_expr) => println!("{prefix} Expr::Conditional"),
    Expr::Func(func) => {
      let prefix = format!("{prefix} Expr::Func");
      describe_func(func, &prefix);
    }
    Expr::Ident(ident) => println!("{prefix} Expr::Ident(name={})", &ident.name),
    Expr::Lit(lit) => {
      let prefix = format!("{prefix} Expr::Lit");
      match lit {
        Lit::Boolean(_lit) => println!("{prefix} Lit::Boolean"),
        Lit::Null => println!("{prefix} Lit::Null"),
        Lit::Number(_lit) => println!("{prefix} Lit::Number"),
        Lit::RegEx(_lit) => println!("{prefix} Lit::RegEx"),
        Lit::String(lit) => {
          let prefix = format!("{prefix} Lit::String");
          match lit {
            StringLit::Double(lit) => {
              if lit.len() < 64 {
                println!(
                  "{prefix} StringLit::Double(len={}, value={})",
                  lit.len(),
                  lit
                )
              } else {
                println!(
                  "{prefix} StringLit::Double(len={}, value=<too long to print>)",
                  lit.len()
                )
              }
            }
            StringLit::Single(lit) => {
              if lit.len() < 64 {
                println!(
                  "{prefix} StringLit::Single(len={}, value={})",
                  lit.len(),
                  lit
                )
              } else {
                println!(
                  "{prefix} StringLit::Single(len={}, value=<too long to print>)",
                  lit.len()
                )
              }
            }
          }
        }
        Lit::Template(_lit) => println!("{prefix} Lit::Template"),
      }
    }
    Expr::Logical(_expr) => println!("{prefix} Expr::Logical"),
    Expr::Member(member_expr) => {
      {
        let prefix = format!(
          "{prefix} Expr::Member(computed={}, object)",
          member_expr.computed
        );
        describe_expr(*member_expr.object, &prefix);
      }
      {
        let prefix = format!(
          "{prefix} Expr::Member(computed={}, property)",
          member_expr.computed
        );
        describe_expr(*member_expr.property, &prefix);
      }
    }
    Expr::MetaProp(_expr) => println!("{prefix} Expr::MetaProp"),
    Expr::New(_expr) => println!("{prefix} Expr::New"),
    Expr::Obj(_expr) => println!("{prefix} Expr::Obj"),
    Expr::Sequence(_expr) => println!("{prefix} Expr::Sequence"),
    Expr::Spread(_expr) => println!("{prefix} Expr::Spread"),
    Expr::Super => println!("{prefix} Expr::Super"),
    Expr::TaggedTemplate(_expr) => println!("{prefix} Expr::TaggedTemplate"),
    Expr::This => println!("{prefix} Expr::This"),
    Expr::Unary(_expr) => println!("{prefix} Expr::Unary"),
    Expr::Update(_expr) => println!("{prefix} Expr::Update"),
    Expr::Yield(_expr) => println!("{prefix} Expr::Yield"),
  }
}
