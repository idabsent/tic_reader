use std::mem;

//TODO realise correct work
pub(crate) fn cast_vec_panic<T, U>(mut input: Vec<T>) -> Vec<U> {
    input.shrink_to_fit();

    let mut m_wrap = mem::ManuallyDrop::new(input);

    let o_len = mem::size_of::<U>();
    let i_len = mem::size_of::<T>();

    let len = if o_len < i_len {
        if i_len % o_len != 0 { panic!("Uncorrect cast. {{Size of input type}} > {{Size of output type}} => {{Size of input type}} % {{Size of output type}} == 0 | i_len: {i_len} ... o_len: {o_len}") }
        i_len / o_len
    } else {
        if m_wrap.len() % o_len != 0 { panic!("Uncorrect cast. {{Size of input type}} < {{Size of output type}} => {{input.len()}} % {{Size of output type}} == 0 | i_len: {i_len} ... o_len: {o_len} ... m_wrap.len(): {}", m_wrap.len()) }
        m_wrap.len() / o_len
    };

    let input = m_wrap.as_mut_ptr();

    println!("len: {len}");

    unsafe {
        Vec::from_raw_parts(input as *mut U, len, len)
    }
}

fn cast_slice<T, U>(input:&[T]) -> &[U]{
    let (_, output, _) = unsafe { input.align_to() };

    output
}

#[cfg(test)]
mod tests {

    #[test]
    #[should_panic]
    fn cast_vec_u8_to_i8_panic() {
        use super::*;

        let in_vec = vec![0x00u8, 0xFF];
        let out_vec: Vec<i16> = cast_vec_panic(in_vec);

        assert_eq!(out_vec, vec![0x00FFi16]);
    }

    #[test]
    fn cast_vec_u8_to_i8() {
        use super::*;

        let mut in_vec = vec![0x00u8, 0xFF];
        in_vec.reverse();
        let out_slice: &[i8] = cast_slice(&in_vec);
        assert_eq!(out_slice, vec![0x00i8, -1].as_slice());
    }

    #[test]
    fn cast_vec_u8_to_u16() {
        use super::*;
        let mut in_vec = vec![0xFFu8, 0x00];
        in_vec.reverse();
        let out_slice: &[u16] = cast_slice(&in_vec);
        assert_eq!(out_slice, vec![0xFF00u16].as_slice());
    }
}
