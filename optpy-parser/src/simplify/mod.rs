mod for_loop;
mod list_comprehension;
mod tuple_assign;

pub(super) use for_loop::simplify_for_loops;
pub(super) use list_comprehension::simplify_list_comprehensions;
pub(super) use tuple_assign::simplify_tuple_assignments;
