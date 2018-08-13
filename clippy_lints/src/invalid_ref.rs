use rustc::lint::*;
use rustc::{declare_lint, lint_array};
use if_chain::if_chain;
use rustc::ty;
use rustc::hir::*;
use crate::utils::{match_def_path, opt_def_id, paths, span_help_and_lint};

/// **What it does:** Checks for creation of references with uninitialized or null value.
///
/// **Why is this bad?** Creation of null references and uninitialized references is undefined
/// behavior, even if they are not dereferenced.
///
/// **Known problems:** None.
///
/// **Example:**
/// ```rust
/// let bad_ref: &usize = std::mem::zeroed();
/// let also_bad_ref: &usize = std::mem::uninitialized();
/// ```
declare_clippy_lint! {
    pub INVALID_REF,
    correctness,
    "creation of invalid reference"
}

const ZERO_REF_SUMMARY: &str = "null reference";
const UNINIT_REF_SUMMARY: &str = "uninitialized reference";
const HELP: &str = "Creation of a null or uninitialized reference is undefined behavior; \
                    see https://doc.rust-lang.org/reference/behavior-considered-undefined.html";

pub struct InvalidRef;

impl LintPass for InvalidRef {
    fn get_lints(&self) -> LintArray {
        lint_array!(INVALID_REF)
    }
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for InvalidRef {
    fn check_expr(&mut self, cx: &LateContext<'a, 'tcx>, expr: &'tcx Expr) {
        if_chain! {
            if let ExprKind::Call(ref path, ref args) = expr.node;
            if let ExprKind::Path(ref qpath) = path.node;
            if args.len() == 0;
            if let ty::TyRef(..) = cx.tables.expr_ty(expr).sty;
            if let Some(def_id) = opt_def_id(cx.tables.qpath_def(qpath, path.hir_id));
            then {
                let msg = if match_def_path(cx.tcx, def_id, &paths::MEM_ZEROED) |
                             match_def_path(cx.tcx, def_id, &paths::INIT)
                {
                    ZERO_REF_SUMMARY
                } else if match_def_path(cx.tcx, def_id, &paths::MEM_UNINIT) |
                          match_def_path(cx.tcx, def_id, &paths::UNINIT)
                {
                    UNINIT_REF_SUMMARY
                } else {
                    return;
                };
                span_help_and_lint(cx, INVALID_REF, expr.span, msg, HELP);
            }
        }
        return;
    }
}
