#[cfg(test)]
mod tests {
    use crate::chord::Chord;

    // #[test]
    //     fn pyton_test() {
    //         let gil = pyo3::Python::acquire_gil();
    //         let py = gil.python();

    //         assert_eq!(
    //             Chord::new("C E G").unwrap().pitched_common_name,
    //             py.eval(
    //                 r#"
    // def test():
    //     import music21
    //     chord = music21.chord.Chord("C E G")
    //     print(chord.pitched_common_name)
    // test()
    // "#,
    //                 None,
    //                 Some(pyo3::types::IntoPyDict::into_py_dict(
    //                     [("your_python_library", py.import("music21").unwrap())],
    //                     py
    //                 )),
    //             )
    //             .unwrap()
    //             .extract::<String>()
    //             .unwrap()
    //         );
    //     }

    // #[test]
    fn c_e_g() {
        let chord = Chord::new("C E G").unwrap();
        assert_eq!(chord.pitched_common_name, "C Major Triad");
    }
}
