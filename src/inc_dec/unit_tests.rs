use super::*;

#[test]
fn pre_inc_positive_returns_incremented_value() {
    // given a mutable value of -42
    let mut sut = -42;

    // when incremented by 1
    let result = sut.pre_inc(1);

    // then the result is -41
    assert_eq!(-41, result);
}

#[test]
fn pre_inc_negative_returns_decremented_value() {
    // given a mutable value of 42
    let mut sut = 42;

    // when incremented by -1
    let result = sut.pre_inc(-1);

    // then the result is 41
    assert_eq!(41, result);
}

#[test]
// -41 is precisely representable in IEEE-754
#[allow(clippy::float_cmp)]
fn post_inc_positive_returns_original_value() {
    // given a mutable value of -42
    let mut sut = -42_f64;

    // when incremented by 1
    let result = sut.post_inc(1_f64);

    // then the result is -42
    assert_eq!(-42_f64, result);

    // and the stored value is -41
    assert_eq!(-41_f64, sut);
}

#[test]
// 41 is precisely representable in IEEE-754
#[allow(clippy::float_cmp)]
fn post_inc_negative_returns_original_value() {
    // given a mutable value of 42
    let mut sut = 42_f64;

    // when incremented by -1
    let result = sut.post_inc(-1_f64);

    // then the result is 41
    assert_eq!(42_f64, result);

    // and the stored value is 41
    assert_eq!(41_f64, sut);

}

#[test]
fn pre_dec_positive_returns_decremented_value() {
    // given a mutable value of -42
    let mut sut = -42;

    // when decremented by 1
    let result = sut.pre_dec(1);

    // then the result is -43
    assert_eq!(-43, result);
}

#[test]
fn pre_dec_negative_returns_incremented_value() {
    // given a mutable value of 42
    let mut sut = 42;

    // when decremented by -1
    let result = sut.pre_dec(-1);

    // then the result is 41
    assert_eq!(43, result);
}

#[test]
// -41 is precisely representable in IEEE-754
#[allow(clippy::float_cmp)]
fn post_dec_positive_returns_original_value() {
    // given a mutable value of -42
    let mut sut = -42_f64;

    // when decremented by 1
    let result = sut.post_dec(1_f64);

    // then the result is -42
    assert_eq!(-42_f64, result);

    // and the stored value is -41
    assert_eq!(-43_f64, sut);
}

#[test]
// 41 is precisely representable in IEEE-754
#[allow(clippy::float_cmp)]
fn post_dec_negative_returns_original_value() {
    // given a mutable value of 42
    let mut sut = 42_f64;

    // when decremented by -1
    let result = sut.post_dec(-1_f64);

    // then the result is 41
    assert_eq!(42_f64, result);

    // and the stored value is 41
    assert_eq!(43_f64, sut);

}