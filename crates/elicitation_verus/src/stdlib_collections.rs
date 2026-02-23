use verus_builtin_macros::verus;

verus! {

// ============================================================================
// Option<T>
// ============================================================================

/// Proof that Option::Some can be constructed.
pub fn verify_option_some<T>(value: T) -> (result: Option<T>)
    ensures result matches Some(v) && v == value,
{
    Some(value)
}

/// Proof that Option::None can be constructed.
pub fn verify_option_none<T>() -> (result: Option<T>)
    ensures result matches None,
{
    None
}

/// Proof that Option::is_some works correctly.
pub fn verify_option_is_some_true<T>(value: T) -> (result: bool)
    ensures result == true,
{
    let opt = Some(value);
    opt.is_some()
}

/// Proof that Option::is_some works for None.
pub fn verify_option_is_none_true<T>() -> (result: bool)
    ensures result == true,
{
    let opt: Option<T> = None;
    opt.is_none()
}

// ============================================================================
// Result<T, E>
// ============================================================================

/// Proof that Result::Ok can be constructed.
pub fn verify_result_ok<T, E>(value: T) -> (result: Result<T, E>)
    ensures result matches Ok(v) && v == value,
{
    Ok(value)
}

/// Proof that Result::Err can be constructed.
pub fn verify_result_err<T, E>(error: E) -> (result: Result<T, E>)
    ensures result matches Err(e) && e == error,
{
    Err(error)
}

/// Proof that Result::is_ok works correctly.
pub fn verify_result_is_ok_true<T, E>(value: T) -> (result: bool)
    ensures result == true,
{
    let res: Result<T, E> = Ok(value);
    res.is_ok()
}

/// Proof that Result::is_err works correctly.
pub fn verify_result_is_err_true<T, E>(error: E) -> (result: bool)
    ensures result == true,
{
    let res: Result<T, E> = Err(error);
    res.is_err()
}

// ============================================================================
// Tuples
// ============================================================================

/// Proof that 2-tuples can be constructed.
pub fn verify_tuple2_construction<T1, T2>(first: T1, second: T2) -> (result: (T1, T2))
    ensures result.0 == first && result.1 == second,
{
    (first, second)
}

/// Proof that 3-tuples can be constructed.
pub fn verify_tuple3_construction<T1, T2, T3>(
    first: T1,
    second: T2,
    third: T3,
) -> (result: (T1, T2, T3))
    ensures result.0 == first && result.1 == second && result.2 == third,
{
    (first, second, third)
}

/// Proof that 4-tuples can be constructed.
pub fn verify_tuple4_construction<T1, T2, T3, T4>(
    first: T1,
    second: T2,
    third: T3,
    fourth: T4,
) -> (result: (T1, T2, T3, T4))
    ensures
        result.0 == first &&
        result.1 == second &&
        result.2 == third &&
        result.3 == fourth,
{
    (first, second, third, fourth)
}

} // verus!
