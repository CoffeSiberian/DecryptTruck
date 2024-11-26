use crate::strucs::data_sii::IDComplexType;
use crate::strucs::float_vector::{
    Int32Vector2, Int32Vector3i32, SingleVector2, SingleVector3, SingleVector4, SingleVector7,
    SingleVector8,
};

pub fn bool_vec_to_string_vec(vec: Vec<bool>) -> Vec<String> {
    vec.into_iter().map(|b| b.to_string()).collect()
}

pub fn i32_vec_to_string_vec(vec: Vec<i32>) -> Vec<String> {
    vec.into_iter().map(|i| i.to_string()).collect()
}

pub fn id_complex_to_string_vec(vec: Vec<IDComplexType>) -> Vec<String> {
    vec.into_iter().map(|id_complex| id_complex.value).collect()
}

pub fn f32_vec_to_string_vec(vec: Vec<f32>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for &value in vec.iter() {
        let mut text = String::new();

        if value.fract() != 0.0 || value >= 1e7 {
            let bytes = value.to_be_bytes();

            for &b in bytes.iter().rev() {
                text = format!("{:02x}{}", b, text);
            }

            text = format!("&{}", text);
        } else {
            text = format!("{:.0}", value as i32);
        }

        res.push(text);
    }

    res
}

pub fn u32_to_string_vec(vec: u32) -> Vec<String> {
    if vec != 4294967295 {
        return vec![vec.to_string()];
    }

    vec!["nil".to_string()]
}

pub fn u64_to_string_vec(vec: u64) -> Vec<String> {
    if vec != 18446744073709551615 {
        return vec![vec.to_string()];
    }

    vec!["nil".to_string()]
}

pub fn u16_to_string_vec(vec: u16) -> Vec<String> {
    if vec != 65535 {
        return vec![vec.to_string()];
    }

    vec!["nil".to_string()]
}

pub fn i16_to_string_vec(vec: i16) -> Vec<String> {
    if vec != 32767 {
        return vec![vec.to_string()];
    }

    vec!["nil".to_string()]
}

pub fn single_to_string(vec: f32) -> String {
    let mut text = String::from("nil");

    if vec.fract() != 0.0 || vec >= 1e7 {
        text.clear();
        let bytes = vec.to_le_bytes();

        for &b in bytes.iter().rev() {
            text.push_str(&format!("{:02x}", b));
        }

        text.insert(0, '&');
    } else {
        text = format!("{:.0}", vec as i32);
    }

    text
}

pub fn u16_vec_to_string_vec(vec: Vec<u16>) -> Vec<String> {
    vec.into_iter().map(|u| u.to_string()).collect()
}

pub fn u32_vec_to_string_vec(vec: Vec<u32>) -> Vec<String> {
    vec.into_iter().map(|u| u.to_string()).collect()
}

pub fn u64_vec_to_string_vec(vec: Vec<u64>) -> Vec<String> {
    vec.into_iter().map(|u| u.to_string()).collect()
}

pub fn vec3_u32_to_string(vec: Int32Vector3i32) -> String {
    format!(
        "({}, {}, {})",
        vec.a.to_string(),
        vec.b.to_string(),
        vec.c.to_string()
    )
}

pub fn vec2_u32_to_string(vec: Int32Vector2) -> String {
    format!("({}, {})", vec.a.to_string(), vec.b.to_string(),)
}

pub fn single_vector_3_to_string(vec: SingleVector3) -> String {
    format!(
        "({}, {}, {})",
        single_to_string(vec.a),
        single_to_string(vec.b),
        single_to_string(vec.c)
    )
}

pub fn single_vector_4_to_string(vec: SingleVector4) -> String {
    format!(
        "({}; {}, {}, {})",
        single_to_string(vec.a),
        single_to_string(vec.b),
        single_to_string(vec.c),
        single_to_string(vec.d)
    )
}

pub fn single_vector_7_to_string(vec: SingleVector7) -> String {
    format!(
        "({}, {}, {}) ({}; {}, {}, {})",
        single_to_string(vec.a),
        single_to_string(vec.b),
        single_to_string(vec.c),
        single_to_string(vec.d),
        single_to_string(vec.e),
        single_to_string(vec.f),
        single_to_string(vec.g)
    )
}

pub fn single_vector_8_to_string(vec: SingleVector8) -> String {
    format!(
        "({}, {}, {}) ({}; {}, {}, {})",
        single_to_string(vec.a),
        single_to_string(vec.b),
        single_to_string(vec.c),
        single_to_string(vec.e),
        single_to_string(vec.f),
        single_to_string(vec.g),
        single_to_string(vec.h)
    )
}

pub fn single_vector_2_string(vec: SingleVector2) -> String {
    format!("({}, {})", single_to_string(vec.a), single_to_string(vec.b))
}

pub fn single_vector_2_vec_to_string_vec(vec: Vec<SingleVector2>) -> Vec<String> {
    let mut string_vec: Vec<String> = Vec::new();

    for single_vec2 in vec {
        string_vec.push(single_vector_2_string(single_vec2));
    }

    string_vec
}

pub fn i32_vec3_to_string_vec(vec: Vec<Int32Vector3i32>) -> Vec<String> {
    let mut string_vec: Vec<String> = Vec::new();

    for i32_vec3 in vec {
        string_vec.push(vec3_u32_to_string(i32_vec3));
    }

    string_vec
}

pub fn single_vector_3_vec_to_string_vec(vec: Vec<SingleVector3>) -> Vec<String> {
    let mut string_vec: Vec<String> = Vec::new();

    for single_vec3 in vec {
        string_vec.push(single_vector_3_to_string(single_vec3));
    }

    string_vec
}

pub fn single_vector_4_vec_to_string_vec(vec: Vec<SingleVector4>) -> Vec<String> {
    let mut string_vec: Vec<String> = Vec::new();

    for single_vec4 in vec {
        string_vec.push(single_vector_4_to_string(single_vec4));
    }

    string_vec
}

pub fn single_vector_7_vec_to_string_vec(vec: Vec<SingleVector7>) -> Vec<String> {
    let mut string_vec: Vec<String> = Vec::new();

    for single_vec7 in vec {
        string_vec.push(single_vector_7_to_string(single_vec7));
    }

    string_vec
}

pub fn single_vector_8_vec_to_string_vec(vec: Vec<SingleVector8>) -> Vec<String> {
    let mut string_vec: Vec<String> = Vec::new();

    for single_vec8 in vec {
        string_vec.push(single_vector_8_to_string(single_vec8));
    }

    string_vec
}

pub fn i64_vec_to_string_vec(vec: Vec<i64>) -> Vec<String> {
    vec.into_iter().map(|i| i.to_string()).collect()
}

pub fn i16_vec_to_string_vec(vec: Vec<i16>) -> Vec<String> {
    vec.into_iter().map(|i| i.to_string()).collect()
}
