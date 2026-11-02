/// Builds a minimal swap trace for a permutation.
///
/// On entry, `ordering[i].0` is the original index that belongs at position
/// `i`. The second field is scratch space. The returned prefix contains the
/// swaps needed to apply the permutation.
pub(crate) fn build_swap_trace(ordering: &mut [(usize, usize)]) -> &mut [(usize, usize)] {
    let len = ordering.len();

    // Build the inverse permutation in the second field.
    for idx in 0..len {
        unsafe {
            // SAFETY: The caller supplies a permutation of 0..len.
            let original = ordering.get_unchecked(idx).0;
            ordering.get_unchecked_mut(original).1 = idx;
        }
    }

    let mut swap_count = 0;

    // Reuse the ordering slice to store the swap trace.
    for i in 0..len {
        unsafe {
            // SAFETY: The permutation and its inverse contain only indices in 0..len.
            let (other, inverse_i) = *ordering.get_unchecked(i);
            if i != other {
                *ordering.get_unchecked_mut(swap_count) = (i, other);
                swap_count += 1;
                if inverse_i > i {
                    ordering.get_unchecked_mut(inverse_i).0 = other;
                    ordering.get_unchecked_mut(other).1 = inverse_i;
                }
            }
        }
    }

    &mut ordering[..swap_count]
}
