use nerd::vector::vector2::*;

#[test]
fn test_vector_new() {
    let v = Vector2::<f32>::new(1., 1.);
    let w = Vector2::<f32>::new(1., 1.);

    assert_eq!(v.x, w.x);
    assert_eq!(v.y, w.y);
}
