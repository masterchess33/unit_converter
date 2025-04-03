use unit_converter::convert;

// Testing the obvious cases for volume

// Milliliter

#[test]
fn ml_to_ml(){
    assert_eq!(convert(15f64, "ml", "ml").unwrap(), "15 ml")
}

#[test]
fn ml_to_l(){
    assert_eq!(convert(13f64, "ml", "l").unwrap(), "0.013 l")
}

#[test]
fn ml_to_m3(){
    assert_eq!(convert(12f64, "ml", "m3").unwrap(), "0.000012 m3")
}

#[test]
fn ml_to_mm3(){
    assert_eq!(convert(14f64, "ml", "mm3").unwrap(), "14000 mm3")
}

// Cubic millimeter

#[test]
fn mm3_to_mm3(){
    assert_eq!(convert(14f64, "mm3", "mm3").unwrap(), "14 mm3")
}

#[test]
fn mm3_to_ml(){
    assert_eq!(convert(9f64, "mm3", "ml").unwrap(), "0.009 ml")
}

#[test]
fn mm3_to_l(){
    assert_eq!(convert(12f64, "mm3", "l").unwrap(), "0.000012 l")
}

#[test]
fn mm3_to_m3(){
    assert_eq!(convert(14f64, "mm3", "m3").unwrap(), "0.000000014 m3")
}

// Liter

#[test]
fn l_to_l(){
    assert_eq!(convert(13f64, "l", "l").unwrap(), "13 l")
}

#[test]
fn l_to_m3(){
    assert_eq!(convert(14f64, "l", "m3").unwrap(), "0.014 m3")
}

#[test]
fn l_to_mm3(){
    assert_eq!(convert(14f64, "l", "mm3").unwrap(), "14000000 mm3")
}

#[test]
fn l_to_ml(){
    assert_eq!(convert(15f64, "l", "ml").unwrap(), "15000 ml")
}

// Cubic meter