use std::mem::MaybeUninit;

pub trait IteratorExtension: Iterator {
    fn eq_by<I: Iterator>(
        &mut self,
        mut other: I,
        mut f: impl FnMut(Self::Item, I::Item) -> bool,
    ) -> bool {
        loop {
            match (self.next(), other.next()) {
                (None, None) => return true,
                (Some(l), Some(r)) => {
                    if !f(l, r) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
    }

    fn array_windows<const N: usize>(self) -> ArrayWindowIterator<Self, Self::Item, N>
    where
        Self: Sized,
    {
        ArrayWindowIterator {
            inner: self,
            filled: 0,
            buffer: [const { MaybeUninit::uninit() }; N],
        }
    }
}

impl<T: Iterator> IteratorExtension for T {}

pub struct ArrayWindowIterator<It, Item, const N: usize> {
    inner: It,
    filled: usize,
    buffer: [MaybeUninit<Item>; N],
}

impl<It, Item, const N: usize> Iterator for ArrayWindowIterator<It, Item, N>
where
    It: Iterator<Item = Item>,
    Item: Clone,
{
    type Item = [Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.filled == N {
            self.filled = 0; // pre-poop
            // safety: all entries are init
            unsafe { self.buffer[0].assume_init_drop() };
            self.buffer.rotate_left(1);
            self.filled = N - 1; // un-poop
        }

        while self.filled < N {
            let item = self.inner.next()?;
            self.buffer[self.filled].write(item);
            self.filled += 1;
        }

        Some({
            self.buffer
                .each_ref()
                .map(|item| unsafe { item.assume_init_ref() }.clone())
        })
    }
}

pub fn search_grid<const N: usize, T>(
    grid: &[Vec<T>],
    is_needle: &impl Fn(&[&T; N]) -> bool,
) -> usize {
    fn count_matches<'a, const N: usize, T: 'a>(
        iter: impl Iterator<Item = impl Iterator<Item = &'a T>>,
        is_needle: &impl Fn(&[&T; N]) -> bool,
    ) -> usize {
        iter.map(|line| {
            line.array_windows::<N>()
                .filter(|window| is_needle(window))
                .count()
        })
        .sum()
    }

    count_matches(horizontal_iter(grid), is_needle)
        + count_matches(vertical_iter(grid), is_needle)
        + count_matches(diag_bl_tr_iter(grid), is_needle)
        + count_matches(diag_tl_br_iter(grid), is_needle)
}

pub fn diag_tl_br_iter<T>(grid: &[Vec<T>]) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    (0..grid.len())
        .map(|row| (row, 0))
        .chain((1..grid[0].len()).map(|column| (0, column)))
        .map(move |(start_row, start_column)| {
            (0..((grid.len() - start_row).min(grid[0].len() - start_column)))
                .map(move |offset| &grid[start_row + offset][start_column + offset])
        })
}

pub fn diag_bl_tr_iter<T>(grid: &[Vec<T>]) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    (0..grid.len())
        .map(|row| (row, 0))
        .chain((1..grid[0].len()).map(|column| (grid.len() - 1, column)))
        .map(move |(start_row, start_column)| {
            (0..((start_row + 1).min(grid[0].len() - start_column)))
                .map(move |offset| &grid[start_row - offset][start_column + offset])
        })
}

pub fn vertical_iter<T>(grid: &[Vec<T>]) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    (0..grid.len()).map(move |column| (0..grid.len()).map(move |row| &grid[row][column]))
}

pub fn horizontal_iter<T>(grid: &[Vec<T>]) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    grid.iter().map(|line| line.iter())
}
