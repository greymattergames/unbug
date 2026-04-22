#![allow(unused_imports)]

use proc_macro_error::{abort, proc_macro_error};
use ::proc_macro::TokenStream;
use ::proc_macro2::{Span, TokenStream as TokenStream2};
use ::quote::{
    format_ident,
    quote,
    quote_spanned,
    ToTokens,
};
use syn::spanned::Spanned;
use ::syn::{*,
    parse::{Parse, Parser, ParseStream},
    punctuated::Punctuated,
    Result,
    visit_mut::{self, VisitMut},
    visit::{self, Visit},
};

// Message helper attribute name
static MSG_ATTR: &str = "fail_msg";

// Ignore helper attribute name
static IGNORE_ATTR: &str = "fail_ignore";

#[derive(Default)]
enum DebugControlKind {
    #[default]
    Break,

    // A message helper: #[fail_msg = "A message to debug"]
    // This sets the message that will be printed from the fail! invocation
    Message(String),

    // A ignore helper: #[fail_ignore]
    // Will skip the statement and won't apply the debug closure
    Ignore,
}

#[derive(Default)]
struct DebugStmtVisitor {
    control: DebugControlKind,
}

impl<'ast> Visit<'ast> for DebugStmtVisitor {
    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        // ignore helper attribute is a Path type e.g. #[path]
        if let Meta::Path(path) = &node.meta && path.is_ident(IGNORE_ATTR) {
            self.control = DebugControlKind::Ignore;
        }

        // message helper attribute is a NameValue type e.g. #[name = value]
        if let Meta::NameValue(meta) = &node.meta {
            if !meta.path.is_ident(MSG_ATTR) {
                return;
            }

            // Only accept strings at the value side
            let Expr::Lit(ExprLit { lit: Lit::Str(str), .. }) = &meta.value else {
                // including the code span (node) with abort to error on the current line
                abort!(node, "Only supports a literal string value");
            };

            self.control = DebugControlKind::Message(str.value());
        };
    }
}

impl VisitMut for DebugStmtVisitor {
    fn visit_expr_try_mut(&mut self, node: &mut syn::ExprTry) {
        self.visit_expr_mut(&mut node.expr);

        let expr = node.expr.clone();

        // spanned applies the node's code span (line numbers) to the quoted tokens,
        // thus the debugging will break at the correct line
        *node = match &self.control {
            // Should have already skipped ignore, but here for completeness
            DebugControlKind::Ignore => return,

            // Handle try expressions without a message
            #[cfg(debug_assertions)]
            DebugControlKind::Break => parse_quote_spanned! {node.question_token.span()=>
                #expr.on_fail(|| ::unbug::_internal::_once!(::unbug::prelude::breakpoint!()))?
            },
            #[cfg(not(debug_assertions))]
            DebugControlKind::Break => parse_quote_spanned! {node.question_token.span()=>
                #expr.try_to_result()?
            },

            // Handle try expressions with a specified message
            DebugControlKind::Message(msg) => parse_quote_spanned! {node.question_token.span()=>
                #expr.on_fail(|| ::unbug::prelude::fail!(#msg))?
            },
        };
    }
}

struct DebugTryVisitor;

impl VisitMut for DebugTryVisitor {
    // We should have started by visiting the whole function
    // Now we are possibly in multiple blocks at various levels
    fn visit_block_mut(&mut self, node: &mut syn::Block) {
        // Visit other blocks
        visit_mut::visit_block_mut(self, node);

        // Handle each statement in a block individually
        // In order to apply control helpers to the whole statement
        for stmt in node.stmts.iter_mut() {
            let mut visitor = DebugStmtVisitor::default();
            // First, determine what debug message to use, if any
            visitor.visit_stmt(stmt);

            // Don't apply the closure if ignore is specified
            if matches!(visitor.control, DebugControlKind::Ignore) {
                return;
            }

            // Apply debugging to the statement
            visitor.visit_stmt_mut(stmt);
        }
    }
}

struct StripAttrsVisitor;

impl StripAttrsVisitor {
    // List of attributes to strip
    const STRIP_ATTRS: [&str; 2] = [
        MSG_ATTR,
        IGNORE_ATTR,
    ];

    fn remove_attrs(attrs: &mut Vec<Attribute>) {
        attrs.retain(|attr| {
            // Keep any attributes that don't match
            !Self::STRIP_ATTRS
                .iter()
                .any(|name| attr.path().is_ident(name))
        });
    }
}

// Types of expressions to remove attributes from
// This list is mostly trial and error
// It may need to be expanded in the case that there is an unsupported macro error
impl VisitMut for StripAttrsVisitor {
    fn visit_expr_try_mut(&mut self, node: &mut syn::ExprTry) {
        visit_mut::visit_expr_try_mut(self, node);
        Self::remove_attrs(&mut node.attrs);
    }

    fn visit_expr_method_call_mut(&mut self, node: &mut syn::ExprMethodCall) {
        visit_mut::visit_expr_method_call_mut(self, node);
        Self::remove_attrs(&mut node.attrs);
    }

    fn visit_expr_return_mut(&mut self, node: &mut syn::ExprReturn) {
        visit_mut::visit_expr_return_mut(self, node);
        Self::remove_attrs(&mut node.attrs);
    }

    fn visit_local_mut(&mut self, node: &mut syn::Local) {
        visit_mut::visit_local_mut(self, node);
        Self::remove_attrs(&mut node.attrs);
    }
}

/// A proc macro to apply to a function that will set every invocation of the try-operator (`?`)
/// To use a closure defined on Result and Option types called `on_fail` (defined in errors.rs)
///
/// ## Example:
///
/// ```rust
/// #[debug_fail]
/// fn my_system(commands: mut Commands) {
///     commands.get_entity(Entity::PLACEHOLDER)?;
///
///     #[fail_msg = "A message to debug"]
///     commands.get_entity(Entity::PLACEHOLDER)?;
///
///     #[fail_ignore]
///     commands.get_entity(Entity::PLACEHOLDER)?;
/// }
/// ```
///
/// ### becomes:
///
/// ```rust
/// #[debug_fail]
/// fn my_system(commands: mut Commands) {
///     commands.get_entity(Entity::PLACEHOLDER).on_fail(|| once!(breakpoint!()))?;
///
///     commands.get_entity(Entity::PLACEHOLDER).on_fail(|| fail!("A message to debug"))?;
///
///     commands.get_entity(Entity::PLACEHOLDER)?;
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn debug_fail(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn: syn::ItemFn = parse_macro_input!(item);

    // Determine debug messages and add the closure call with a fail breakpoint
    // Start by visiting the whole function
    // The visitor will delegate out to individual blocks and then statements
    let mut visitor = DebugTryVisitor;
    visitor.visit_item_fn_mut(&mut item_fn);

    // Strip helper attributes from the function body where they would cause a syntax error
    let mut attr_stripper = StripAttrsVisitor;
    attr_stripper.visit_item_fn_mut(&mut item_fn);

    item_fn.into_token_stream().into()
}

