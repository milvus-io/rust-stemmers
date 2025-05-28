//! Generated from indonesian.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 3] = &[
    Among("kah", -1, 1, None),
    Among("lah", -1, 1, None),
    Among("pun", -1, 1, None),
];

static A_1: &'static [Among<Context>; 3] = &[
    Among("nya", -1, 1, None),
    Among("ku", -1, 1, None),
    Among("mu", -1, 1, None),
];

static A_2: &'static [Among<Context>; 3] = &[
    Among("i", -1, 1, Some(&r_SUFFIX_I_OK)),
    Among("an", -1, 1, Some(&r_SUFFIX_AN_OK)),
    Among("kan", 1, 1, Some(&r_SUFFIX_KAN_OK)),
];

static A_3: &'static [Among<Context>; 12] = &[
    Among("di", -1, 1, None),
    Among("ke", -1, 2, None),
    Among("me", -1, 1, None),
    Among("mem", 2, 5, None),
    Among("men", 2, 1, None),
    Among("meng", 4, 1, None),
    Among("meny", 4, 3, Some(&r_VOWEL)),
    Among("pem", -1, 6, None),
    Among("pen", -1, 2, None),
    Among("peng", 8, 2, None),
    Among("peny", 8, 4, Some(&r_VOWEL)),
    Among("ter", -1, 1, None),
];

static A_4: &'static [Among<Context>; 6] = &[
    Among("be", -1, 3, Some(&r_KER)),
    Among("belajar", 0, 4, None),
    Among("ber", 0, 3, None),
    Among("pe", -1, 1, None),
    Among("pelajar", 3, 2, None),
    Among("per", 3, 1, None),
];

static G_vowel: &'static [u8; 3] = &[17, 65, 16];

#[derive(Clone)]
struct Context {
    i_prefix: i32,
    i_measure: i32,
}

fn r_remove_particle(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 2 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 104 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 110 as u8)) {
        return false;
    }

    if env.find_among_b(A_0, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !env.slice_del() {
        return false;
    }
    context.i_measure -= 1;
    return true
}

fn r_remove_possessive_pronoun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 97 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 117 as u8)) {
        return false;
    }

    if env.find_among_b(A_1, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !env.slice_del() {
        return false;
    }
    context.i_measure -= 1;
    return true
}

fn r_SUFFIX_KAN_OK(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if context.i_prefix == 3{
        return false;
    }
    if context.i_prefix == 2{
        return false;
    }
    return true
}

fn r_SUFFIX_AN_OK(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_prefix != 1
}

fn r_SUFFIX_I_OK(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if context.i_prefix > 2{
        return false;
    }
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"s") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_remove_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 105 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 110 as u8)) {
        return false;
    }

    if env.find_among_b(A_2, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !env.slice_del() {
        return false;
    }
    context.i_measure -= 1;
    return true
}

fn r_VOWEL(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !env.in_grouping(G_vowel, 97, 117) {
        return false;
    }
    return true
}

fn r_KER(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !env.out_grouping(G_vowel, 97, 117) {
        return false;
    }
    if !env.eq_s(&"er") {
        return false;
    }
    return true
}

fn r_remove_first_order_prefix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    if (env.cursor + 1 >= env.limit || (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 105 as u8 && env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 101 as u8)) {
        return false;
    }

    among_var = env.find_among(A_3, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    match among_var {
        1 => {
            if !env.slice_del() {
                return false;
            }
            context.i_prefix = 1;
            context.i_measure -= 1;
        }
        2 => {
            if !env.slice_del() {
                return false;
            }
            context.i_prefix = 3;
            context.i_measure -= 1;
        }
        3 => {
            context.i_prefix = 1;
            if !env.slice_from("s") {
                return false;
            }
            context.i_measure -= 1;
        }
        4 => {
            context.i_prefix = 3;
            if !env.slice_from("s") {
                return false;
            }
            context.i_measure -= 1;
        }
        5 => {
            context.i_prefix = 1;
            context.i_measure -= 1;
            'lab0: loop {
                let v_1 = env.cursor;
                'lab1: loop {
                    let v_2 = env.cursor;
                    if !env.in_grouping(G_vowel, 97, 117) {
                        break 'lab1;
                    }
                    env.cursor = v_2;
                    if !env.slice_from("p") {
                        return false;
                    }
                    break 'lab0;
                }
                env.cursor = v_1;
                if !env.slice_del() {
                    return false;
                }
                break 'lab0;
            }
        }
        6 => {
            context.i_prefix = 3;
            context.i_measure -= 1;
            'lab2: loop {
                let v_3 = env.cursor;
                'lab3: loop {
                    let v_4 = env.cursor;
                    if !env.in_grouping(G_vowel, 97, 117) {
                        break 'lab3;
                    }
                    env.cursor = v_4;
                    if !env.slice_from("p") {
                        return false;
                    }
                    break 'lab2;
                }
                env.cursor = v_3;
                if !env.slice_del() {
                    return false;
                }
                break 'lab2;
            }
        }
        _ => ()
    }
    return true
}

fn r_remove_second_order_prefix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    if (env.cursor + 1 >= env.limit || env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 101 as u8) {
        return false;
    }

    among_var = env.find_among(A_4, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    match among_var {
        1 => {
            if !env.slice_del() {
                return false;
            }
            context.i_prefix = 2;
            context.i_measure -= 1;
        }
        2 => {
            if !env.slice_from("ajar") {
                return false;
            }
            context.i_measure -= 1;
        }
        3 => {
            if !env.slice_del() {
                return false;
            }
            context.i_prefix = 4;
            context.i_measure -= 1;
        }
        4 => {
            if !env.slice_from("ajar") {
                return false;
            }
            context.i_prefix = 4;
            context.i_measure -= 1;
        }
        _ => ()
    }
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_prefix: 0,
        i_measure: 0,
    };
    context.i_measure = 0;
    let v_1 = env.cursor;
    'lab0: loop {
        'replab1: loop{
            let v_2 = env.cursor;
            'lab2: for _ in 0..1 {
                if !env.go_out_grouping(G_vowel, 97, 117) {
                    break 'lab2;
                }
env.next_char();                context.i_measure += 1;
                continue 'replab1;
            }
            env.cursor = v_2;
            break 'replab1;
        }
        break 'lab0;
    }
    env.cursor = v_1;
    if context.i_measure <= 2{
        return false;
    }
    context.i_prefix = 0;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_3 = env.limit - env.cursor;
    r_remove_particle(env, context);
    env.cursor = env.limit - v_3;
    if context.i_measure <= 2{
        return false;
    }
    let v_4 = env.limit - env.cursor;
    r_remove_possessive_pronoun(env, context);
    env.cursor = env.limit - v_4;
    env.cursor = env.limit_backward;
    if context.i_measure <= 2{
        return false;
    }
    'lab3: loop {
        let v_5 = env.cursor;
        'lab4: loop {
            let v_6 = env.cursor;
            if !r_remove_first_order_prefix(env, context) {
                break 'lab4;
            }
            let v_7 = env.cursor;
            'lab5: loop {
                let v_8 = env.cursor;
                if context.i_measure <= 2{
                    break 'lab5;
                }
                env.limit_backward = env.cursor;
                env.cursor = env.limit;
                if !r_remove_suffix(env, context) {
                    break 'lab5;
                }
                env.cursor = env.limit_backward;
                env.cursor = v_8;
                if context.i_measure <= 2{
                    break 'lab5;
                }
                if !r_remove_second_order_prefix(env, context) {
                    break 'lab5;
                }
                break 'lab5;
            }
            env.cursor = v_7;
            env.cursor = v_6;
            break 'lab3;
        }
        env.cursor = v_5;
        let v_9 = env.cursor;
        r_remove_second_order_prefix(env, context);
        env.cursor = v_9;
        let v_10 = env.cursor;
        'lab6: loop {
            if context.i_measure <= 2{
                break 'lab6;
            }
            env.limit_backward = env.cursor;
            env.cursor = env.limit;
            if !r_remove_suffix(env, context) {
                break 'lab6;
            }
            env.cursor = env.limit_backward;
            break 'lab6;
        }
        env.cursor = v_10;
        break 'lab3;
    }
    return true
}
