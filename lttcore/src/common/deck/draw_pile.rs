use std::ops::Deref;
use std::sync::Arc;

/// Designed to provide a structurally shared draw pile. Once created the draw pile is
/// immutable, but it can be cloned very cheaply. DrawPile also deref's to a slice of T
/// so it can be used anywhere you need an immutable slice
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawPile<T: Clone> {
    pile: Arc<Vec<T>>,
    offset: usize,
}

impl<T: Clone> From<Vec<T>> for DrawPile<T> {
    fn from(pile: Vec<T>) -> Self {
        Self {
            pile: Arc::new(pile),
            offset: 0,
        }
    }
}

impl<T: Clone> DrawPile<T> {
    pub fn draw(&mut self) -> Option<T> {
        let next = self.pile.get(self.offset).cloned();
        self.offset += 1;
        next
    }
}

impl<T: Clone> Deref for DrawPile<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.pile[self.offset..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_you_can_draw_from_the_draw_pile() {
        let mut pile: DrawPile<usize> = vec![1, 2, 3].into();
        assert_eq!(pile.deref(), &[1, 2, 3]);
        assert_eq!(pile.len(), 3);

        assert!(pile.contains(&1));
        assert_eq!(pile.draw(), Some(1));
        assert!(!pile.contains(&1));

        assert_eq!(pile.deref(), &[2, 3]);
        assert_eq!(pile.len(), 2);
        assert_eq!(pile.draw(), Some(2));

        assert_eq!(pile.deref(), &[3]);
        assert_eq!(pile.len(), 1);
        assert_eq!(pile.draw(), Some(3));

        assert_eq!(pile.deref(), &[]);
        assert_eq!(pile.len(), 0);
        assert_eq!(pile.draw(), None);
    }
}
