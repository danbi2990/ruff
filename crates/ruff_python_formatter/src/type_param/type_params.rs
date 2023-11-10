use ruff_formatter::FormatResult;
use ruff_python_ast::AstNode;
use ruff_python_ast::TypeParams;
use ruff_text_size::Ranged;

use crate::builders::PyFormatterExtensions;
use crate::comments::SourceComment;
use crate::expression::parentheses::parenthesized;
use crate::prelude::*;

#[derive(Default)]
pub struct FormatTypeParams;

/// Formats a sequence of [`TypeParam`] nodes.
impl FormatNodeRule<TypeParams> for FormatTypeParams {
    fn fmt_fields(&self, item: &TypeParams, f: &mut PyFormatter) -> FormatResult<()> {
        // A dangling comment indicates a comment on the same line as the opening bracket, e.g.:
        // ```python
        // type foo[  # This type parameter clause has a dangling comment.
        //     a,
        //     b,
        //     c,
        // ] = ...
        let comments = f.context().comments().clone();
        let dangling_comments = comments.dangling(item.as_any_node_ref());

        let items = format_with(|f| {
            f.join_comma_separated(item.end())
                .nodes(item.type_params.iter())
                .finish()
        });

        parenthesized("[", &items, "]")
            .with_dangling_comments(dangling_comments)
            .fmt(f)
    }

    fn fmt_dangling_comments(
        &self,
        _dangling_comments: &[SourceComment],
        _f: &mut PyFormatter,
    ) -> FormatResult<()> {
        // Handled in `fmt_fields`
        Ok(())
    }
}
