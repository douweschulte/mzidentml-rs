use std::slice::Iter;

use super::is_element::IsElement;

pub trait IsList<'a, E>
where
    E: IsElement,
{
    fn list_items(&self) -> &[E];

    fn iter(&'a self) -> Iter<'a, E> {
        self.list_items().iter()
    }

    fn is_empty(&self) -> bool {
        self.list_items().is_empty()
    }

    fn len(&self) -> usize {
        self.list_items().len()
    }
}
