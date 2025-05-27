//! Generated from dutch.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 21] = &[
    Among("a", -1, 1, None),
    Among("e", -1, 2, None),
    Among("o", -1, 1, None),
    Among("u", -1, 1, None),
    Among("à", -1, 1, None),
    Among("á", -1, 1, None),
    Among("â", -1, 1, None),
    Among("ä", -1, 1, None),
    Among("è", -1, 2, None),
    Among("é", -1, 2, None),
    Among("ê", -1, 2, None),
    Among("eë", -1, 3, None),
    Among("ië", -1, 4, None),
    Among("ò", -1, 1, None),
    Among("ó", -1, 1, None),
    Among("ô", -1, 1, None),
    Among("ö", -1, 1, None),
    Among("ù", -1, 1, None),
    Among("ú", -1, 1, None),
    Among("û", -1, 1, None),
    Among("ü", -1, 1, None),
];

static A_1: &'static [Among<Context>; 8] = &[
    Among("nde", -1, 8, None),
    Among("en", -1, 7, None),
    Among("s", -1, 2, None),
    Among("'s", 2, 1, None),
    Among("es", 2, 4, None),
    Among("ies", 4, 3, None),
    Among("aus", 2, 6, None),
    Among("és", 2, 5, None),
];

static A_2: &'static [Among<Context>; 11] = &[
    Among("de", -1, 5, None),
    Among("ge", -1, 2, None),
    Among("ische", -1, 4, None),
    Among("je", -1, 1, None),
    Among("lijke", -1, 3, None),
    Among("le", -1, 9, None),
    Among("ene", -1, 10, None),
    Among("re", -1, 8, None),
    Among("se", -1, 7, None),
    Among("te", -1, 6, None),
    Among("ieve", -1, 11, None),
];

static A_3: &'static [Among<Context>; 14] = &[
    Among("heid", -1, 3, None),
    Among("fie", -1, 7, None),
    Among("gie", -1, 8, None),
    Among("atie", -1, 1, None),
    Among("isme", -1, 5, None),
    Among("ing", -1, 5, None),
    Among("arij", -1, 6, None),
    Among("erij", -1, 5, None),
    Among("sel", -1, 3, None),
    Among("rder", -1, 4, None),
    Among("ster", -1, 3, None),
    Among("iteit", -1, 2, None),
    Among("dst", -1, 10, None),
    Among("tst", -1, 9, None),
];

static A_4: &'static [Among<Context>; 16] = &[
    Among("end", -1, 9, None),
    Among("atief", -1, 2, None),
    Among("erig", -1, 9, None),
    Among("achtig", -1, 3, None),
    Among("ioneel", -1, 1, None),
    Among("baar", -1, 3, None),
    Among("laar", -1, 5, None),
    Among("naar", -1, 4, None),
    Among("raar", -1, 6, None),
    Among("eriger", -1, 9, None),
    Among("achtiger", -1, 3, None),
    Among("lijker", -1, 8, None),
    Among("tant", -1, 7, None),
    Among("erigst", -1, 9, None),
    Among("achtigst", -1, 3, None),
    Among("lijkst", -1, 8, None),
];

static A_5: &'static [Among<Context>; 3] = &[
    Among("ig", -1, 1, None),
    Among("iger", -1, 1, None),
    Among("igst", -1, 1, None),
];

static A_6: &'static [Among<Context>; 3] = &[
    Among("ft", -1, 2, None),
    Among("kt", -1, 1, None),
    Among("pt", -1, 3, None),
];

static A_7: &'static [Among<Context>; 22] = &[
    Among("bb", -1, 1, None),
    Among("cc", -1, 2, None),
    Among("dd", -1, 3, None),
    Among("ff", -1, 4, None),
    Among("gg", -1, 5, None),
    Among("hh", -1, 6, None),
    Among("jj", -1, 7, None),
    Among("kk", -1, 8, None),
    Among("ll", -1, 9, None),
    Among("mm", -1, 10, None),
    Among("nn", -1, 11, None),
    Among("pp", -1, 12, None),
    Among("qq", -1, 13, None),
    Among("rr", -1, 14, None),
    Among("ss", -1, 15, None),
    Among("tt", -1, 16, None),
    Among("v", -1, 4, None),
    Among("vv", 16, 17, None),
    Among("ww", -1, 18, None),
    Among("xx", -1, 19, None),
    Among("z", -1, 15, None),
    Among("zz", 20, 20, None),
];

static A_8: &'static [Among<Context>; 2] = &[
    Among("d", -1, 1, None),
    Among("t", -1, 2, None),
];

static A_9: &'static [Among<Context>; 6] = &[
    Among("", -1, -1, None),
    Among("eft", 0, 1, None),
    Among("vaa", 0, 1, None),
    Among("val", 0, 1, None),
    Among("vali", 3, -1, None),
    Among("vare", 0, 1, None),
];

static A_10: &'static [Among<Context>; 2] = &[
    Among("ë", -1, 1, None),
    Among("ï", -1, 2, None),
];

static A_11: &'static [Among<Context>; 2] = &[
    Among("ë", -1, 1, None),
    Among("ï", -1, 2, None),
];

static G_E: &'static [u8; 17] = &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120];

static G_AIOU: &'static [u8; 20] = &[1, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 11, 120, 46, 15];

static G_AEIOU: &'static [u8; 20] = &[17, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 139, 127, 46, 15];

static G_v: &'static [u8; 20] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 139, 127, 46, 15];

static G_v_WX: &'static [u8; 20] = &[17, 65, 208, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 139, 127, 46, 15];

#[derive(Clone)]
struct Context {
    b_GE_removed: bool,
    b_stemmed: bool,
    i_p2: i32,
    i_p1: i32,
    S_ch: String,
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p2 <= env.cursor
}

fn r_V(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            if !env.in_grouping_b(G_v, 97, 252) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"ij") {
            return false;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_VX(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if env.cursor <= env.limit_backward {
        return false;
    }
    env.previous_char();
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            if !env.in_grouping_b(G_v, 97, 252) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"ij") {
            return false;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_C(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"ij") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    if !env.out_grouping_b(G_v, 97, 252) {
        return false;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_lengthen_V(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        if !env.out_grouping_b(G_v_WX, 97, 252) {
            break 'lab0;
        }
        env.ket = env.cursor;
        among_var = env.find_among_b(A_0, context);
        if among_var == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        match among_var {
            1 => {
                let v_2 = env.limit - env.cursor;
                'lab1: loop {
                    let v_3 = env.limit - env.cursor;
                    'lab2: loop {
                        if !env.out_grouping_b(G_AEIOU, 97, 252) {
                            break 'lab2;
                        }
                        break 'lab1;
                    }
                    env.cursor = env.limit - v_3;
                    if env.cursor > env.limit_backward {
                        break 'lab0;
                    }
                    break 'lab1;
                }
                env.cursor = env.limit - v_2;
                context.S_ch = env.slice_to();
                if context.S_ch.is_empty() {
                    return false;
                }
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, &context.S_ch);
                env.cursor = c;
            }
            2 => {
                let v_4 = env.limit - env.cursor;
                'lab3: loop {
                    let v_5 = env.limit - env.cursor;
                    'lab4: loop {
                        if !env.out_grouping_b(G_AEIOU, 97, 252) {
                            break 'lab4;
                        }
                        break 'lab3;
                    }
                    env.cursor = env.limit - v_5;
                    if env.cursor > env.limit_backward {
                        break 'lab0;
                    }
                    break 'lab3;
                }
                let v_6 = env.limit - env.cursor;
                'lab5: loop {
                    'lab6: loop {
                        let v_7 = env.limit - env.cursor;
                        'lab7: loop {
                            if !env.in_grouping_b(G_AIOU, 97, 252) {
                                break 'lab7;
                            }
                            break 'lab6;
                        }
                        env.cursor = env.limit - v_7;
                        if !env.in_grouping_b(G_E, 101, 235) {
                            break 'lab5;
                        }
                        if env.cursor > env.limit_backward {
                            break 'lab5;
                        }
                        break 'lab6;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_6;
                let v_8 = env.limit - env.cursor;
                'lab8: loop {
                    if env.cursor <= env.limit_backward {
                        break 'lab8;
                    }
                    env.previous_char();
                    if !env.in_grouping_b(G_AIOU, 97, 252) {
                        break 'lab8;
                    }
                    if !env.out_grouping_b(G_AEIOU, 97, 252) {
                        break 'lab8;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_8;
                env.cursor = env.limit - v_4;
                context.S_ch = env.slice_to();
                if context.S_ch.is_empty() {
                    return false;
                }
                let c = env.cursor;
                let (bra, ket) = (env.cursor, env.cursor);
                env.insert(bra, ket, &context.S_ch);
                env.cursor = c;
            }
            3 => {
                if !env.slice_from("eëe") {
                    return false;
                }
            }
            4 => {
                if !env.slice_from("iee") {
                    return false;
                }
            }
            _ => ()
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_Step_1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((540704 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_1, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !env.slice_del() {
                return false;
            }
        }
        2 => {
            if !r_R1(env, context) {
                return false;
            }
            let v_1 = env.limit - env.cursor;
            'lab0: loop {
                if !env.eq_s_b(&"t") {
                    break 'lab0;
                }
                if !r_R1(env, context) {
                    break 'lab0;
                }
                return false;
            }
            env.cursor = env.limit - v_1;
            if !r_C(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
        }
        3 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_from("ie") {
                return false;
            }
        }
        4 => {
            'lab1: loop {
                let v_2 = env.limit - env.cursor;
                'lab2: loop {
                    let v_3 = env.limit - env.cursor;
                    if !env.eq_s_b(&"ar") {
                        break 'lab2;
                    }
                    if !r_R1(env, context) {
                        break 'lab2;
                    }
                    if !r_C(env, context) {
                        break 'lab2;
                    }
                    env.cursor = env.limit - v_3;
                    if !env.slice_del() {
                        return false;
                    }
                    r_lengthen_V(env, context);
                    break 'lab1;
                }
                env.cursor = env.limit - v_2;
                'lab3: loop {
                    let v_4 = env.limit - env.cursor;
                    if !env.eq_s_b(&"er") {
                        break 'lab3;
                    }
                    if !r_R1(env, context) {
                        break 'lab3;
                    }
                    if !r_C(env, context) {
                        break 'lab3;
                    }
                    env.cursor = env.limit - v_4;
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab1;
                }
                env.cursor = env.limit - v_2;
                if !r_R1(env, context) {
                    return false;
                }
                if !r_C(env, context) {
                    return false;
                }
                if !env.slice_from("e") {
                    return false;
                }
                break 'lab1;
            }
        }
        5 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_from("é") {
                return false;
            }
        }
        6 => {
            if !r_R1(env, context) {
                return false;
            }
            if !r_V(env, context) {
                return false;
            }
            if !env.slice_from("au") {
                return false;
            }
        }
        7 => {
            'lab4: loop {
                let v_5 = env.limit - env.cursor;
                'lab5: loop {
                    if !env.eq_s_b(&"hed") {
                        break 'lab5;
                    }
                    if !r_R1(env, context) {
                        break 'lab5;
                    }
                    env.bra = env.cursor;
                    if !env.slice_from("heid") {
                        return false;
                    }
                    break 'lab4;
                }
                env.cursor = env.limit - v_5;
                'lab6: loop {
                    if !env.eq_s_b(&"nd") {
                        break 'lab6;
                    }
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab4;
                }
                env.cursor = env.limit - v_5;
                'lab7: loop {
                    if !env.eq_s_b(&"d") {
                        break 'lab7;
                    }
                    if !r_R1(env, context) {
                        break 'lab7;
                    }
                    if !r_C(env, context) {
                        break 'lab7;
                    }
                    env.bra = env.cursor;
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab4;
                }
                env.cursor = env.limit - v_5;
                'lab8: loop {
                    'lab9: loop {
                        let v_6 = env.limit - env.cursor;
                        'lab10: loop {
                            if !env.eq_s_b(&"i") {
                                break 'lab10;
                            }
                            break 'lab9;
                        }
                        env.cursor = env.limit - v_6;
                        if !env.eq_s_b(&"j") {
                            break 'lab8;
                        }
                        break 'lab9;
                    }
                    if !r_V(env, context) {
                        break 'lab8;
                    }
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab4;
                }
                env.cursor = env.limit - v_5;
                if !r_R1(env, context) {
                    return false;
                }
                if !r_C(env, context) {
                    return false;
                }
                if !env.slice_del() {
                    return false;
                }
                r_lengthen_V(env, context);
                break 'lab4;
            }
        }
        8 => {
            if !env.slice_from("nd") {
                return false;
            }
        }
        _ => ()
    }
    return true
}

fn r_Step_2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8) {
        return false;
    }

    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"'t") {
                        break 'lab1;
                    }
                    env.bra = env.cursor;
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                'lab2: loop {
                    if !env.eq_s_b(&"et") {
                        break 'lab2;
                    }
                    env.bra = env.cursor;
                    if !r_R1(env, context) {
                        break 'lab2;
                    }
                    if !r_C(env, context) {
                        break 'lab2;
                    }
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                'lab3: loop {
                    if !env.eq_s_b(&"rnt") {
                        break 'lab3;
                    }
                    env.bra = env.cursor;
                    if !env.slice_from("rn") {
                        return false;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                'lab4: loop {
                    if !env.eq_s_b(&"t") {
                        break 'lab4;
                    }
                    env.bra = env.cursor;
                    if !r_R1(env, context) {
                        break 'lab4;
                    }
                    if !r_VX(env, context) {
                        break 'lab4;
                    }
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                'lab5: loop {
                    if !env.eq_s_b(&"ink") {
                        break 'lab5;
                    }
                    env.bra = env.cursor;
                    if !env.slice_from("ing") {
                        return false;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                'lab6: loop {
                    if !env.eq_s_b(&"mp") {
                        break 'lab6;
                    }
                    env.bra = env.cursor;
                    if !env.slice_from("m") {
                        return false;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                'lab7: loop {
                    if !env.eq_s_b(&"'") {
                        break 'lab7;
                    }
                    env.bra = env.cursor;
                    if !r_R1(env, context) {
                        break 'lab7;
                    }
                    if !env.slice_del() {
                        return false;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                env.bra = env.cursor;
                if !r_R1(env, context) {
                    return false;
                }
                if !r_C(env, context) {
                    return false;
                }
                if !env.slice_del() {
                    return false;
                }
                break 'lab0;
            }
        }
        2 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_from("g") {
                return false;
            }
        }
        3 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_from("lijk") {
                return false;
            }
        }
        4 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_from("isch") {
                return false;
            }
        }
        5 => {
            if !r_R1(env, context) {
                return false;
            }
            if !r_C(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
        }
        6 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_from("t") {
                return false;
            }
        }
        7 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_from("s") {
                return false;
            }
        }
        8 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_from("r") {
                return false;
            }
        }
        9 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
            let (bra, ket) = (env.cursor, env.cursor);
            env.insert(bra, ket, "l");
            r_lengthen_V(env, context);
        }
        10 => {
            if !r_R1(env, context) {
                return false;
            }
            if !r_C(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
            let (bra, ket) = (env.cursor, env.cursor);
            env.insert(bra, ket, "en");
            r_lengthen_V(env, context);
        }
        11 => {
            if !r_R1(env, context) {
                return false;
            }
            if !r_C(env, context) {
                return false;
            }
            if !env.slice_from("ief") {
                return false;
            }
        }
        _ => ()
    }
    return true
}

fn r_Step_3(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1316016 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_3, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_from("eer") {
                return false;
            }
        }
        2 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
            r_lengthen_V(env, context);
        }
        3 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
        }
        4 => {
            if !env.slice_from("r") {
                return false;
            }
        }
        5 => {
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"ild") {
                        break 'lab1;
                    }
                    if !env.slice_from("er") {
                        return false;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                if !r_R1(env, context) {
                    return false;
                }
                if !env.slice_del() {
                    return false;
                }
                r_lengthen_V(env, context);
                break 'lab0;
            }
        }
        6 => {
            if !r_R1(env, context) {
                return false;
            }
            if !r_C(env, context) {
                return false;
            }
            if !env.slice_from("aar") {
                return false;
            }
        }
        7 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
            let (bra, ket) = (env.cursor, env.cursor);
            env.insert(bra, ket, "f");
            r_lengthen_V(env, context);
        }
        8 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.slice_del() {
                return false;
            }
            let (bra, ket) = (env.cursor, env.cursor);
            env.insert(bra, ket, "g");
            r_lengthen_V(env, context);
        }
        9 => {
            if !r_R1(env, context) {
                return false;
            }
            if !r_C(env, context) {
                return false;
            }
            if !env.slice_from("t") {
                return false;
            }
        }
        10 => {
            if !r_R1(env, context) {
                return false;
            }
            if !r_C(env, context) {
                return false;
            }
            if !env.slice_from("d") {
                return false;
            }
        }
        _ => ()
    }
    return true
}

fn r_Step_4(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            env.ket = env.cursor;
            if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1315024 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
                break 'lab1;
            }

            among_var = env.find_among_b(A_4, context);
            if among_var == 0 {
                break 'lab1;
            }
            env.bra = env.cursor;
            match among_var {
                1 => {
                    if !r_R1(env, context) {
                        break 'lab1;
                    }
                    if !env.slice_from("ie") {
                        return false;
                    }
                }
                2 => {
                    if !r_R1(env, context) {
                        break 'lab1;
                    }
                    if !env.slice_from("eer") {
                        return false;
                    }
                }
                3 => {
                    if !r_R1(env, context) {
                        break 'lab1;
                    }
                    if !env.slice_del() {
                        return false;
                    }
                }
                4 => {
                    if !r_R1(env, context) {
                        break 'lab1;
                    }
                    if !r_V(env, context) {
                        break 'lab1;
                    }
                    if !env.slice_from("n") {
                        return false;
                    }
                }
                5 => {
                    if !r_R1(env, context) {
                        break 'lab1;
                    }
                    if !r_V(env, context) {
                        break 'lab1;
                    }
                    if !env.slice_from("l") {
                        return false;
                    }
                }
                6 => {
                    if !r_R1(env, context) {
                        break 'lab1;
                    }
                    if !r_V(env, context) {
                        break 'lab1;
                    }
                    if !env.slice_from("r") {
                        return false;
                    }
                }
                7 => {
                    if !r_R1(env, context) {
                        break 'lab1;
                    }
                    if !env.slice_from("teer") {
                        return false;
                    }
                }
                8 => {
                    if !r_R1(env, context) {
                        break 'lab1;
                    }
                    if !env.slice_from("lijk") {
                        return false;
                    }
                }
                9 => {
                    if !r_R1(env, context) {
                        break 'lab1;
                    }
                    if !r_C(env, context) {
                        break 'lab1;
                    }
                    if !env.slice_del() {
                        return false;
                    }
                    r_lengthen_V(env, context);
                }
                _ => ()
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        env.ket = env.cursor;
        if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1310848 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            return false;
        }

        if env.find_among_b(A_5, context) == 0 {
            return false;
        }
        env.bra = env.cursor;
        if !r_R1(env, context) {
            return false;
        }
        let v_2 = env.limit - env.cursor;
        'lab2: loop {
            if !env.eq_s_b(&"inn") {
                break 'lab2;
            }
            if env.cursor > env.limit_backward {
                break 'lab2;
            }
            return false;
        }
        env.cursor = env.limit - v_2;
        if !r_C(env, context) {
            return false;
        }
        if !env.slice_del() {
            return false;
        }
        r_lengthen_V(env, context);
        break 'lab0;
    }
    return true
}

fn r_Step_7(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 116 as u8) {
        return false;
    }

    among_var = env.find_among_b(A_6, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !env.slice_from("k") {
                return false;
            }
        }
        2 => {
            if !env.slice_from("f") {
                return false;
            }
        }
        3 => {
            if !env.slice_from("p") {
                return false;
            }
        }
        _ => ()
    }
    return true
}

fn r_Step_6(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((98532828 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_7, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !env.slice_from("b") {
                return false;
            }
        }
        2 => {
            if !env.slice_from("c") {
                return false;
            }
        }
        3 => {
            if !env.slice_from("d") {
                return false;
            }
        }
        4 => {
            if !env.slice_from("f") {
                return false;
            }
        }
        5 => {
            if !env.slice_from("g") {
                return false;
            }
        }
        6 => {
            if !env.slice_from("h") {
                return false;
            }
        }
        7 => {
            if !env.slice_from("j") {
                return false;
            }
        }
        8 => {
            if !env.slice_from("k") {
                return false;
            }
        }
        9 => {
            if !env.slice_from("l") {
                return false;
            }
        }
        10 => {
            if !env.slice_from("m") {
                return false;
            }
        }
        11 => {
            let v_1 = env.limit - env.cursor;
            'lab0: loop {
                if !env.eq_s_b(&"i") {
                    break 'lab0;
                }
                if env.cursor > env.limit_backward {
                    break 'lab0;
                }
                return false;
            }
            env.cursor = env.limit - v_1;
            if !env.slice_from("n") {
                return false;
            }
        }
        12 => {
            if !env.slice_from("p") {
                return false;
            }
        }
        13 => {
            if !env.slice_from("q") {
                return false;
            }
        }
        14 => {
            if !env.slice_from("r") {
                return false;
            }
        }
        15 => {
            if !env.slice_from("s") {
                return false;
            }
        }
        16 => {
            if !env.slice_from("t") {
                return false;
            }
        }
        17 => {
            if !env.slice_from("v") {
                return false;
            }
        }
        18 => {
            if !env.slice_from("w") {
                return false;
            }
        }
        19 => {
            if !env.slice_from("x") {
                return false;
            }
        }
        20 => {
            if !env.slice_from("z") {
                return false;
            }
        }
        _ => ()
    }
    return true
}

fn r_Step_1c(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 100 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 116 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_8, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    if !r_C(env, context) {
        return false;
    }
    match among_var {
        1 => {
            let v_1 = env.limit - env.cursor;
            'lab0: loop {
                if !env.eq_s_b(&"n") {
                    break 'lab0;
                }
                if !r_R1(env, context) {
                    break 'lab0;
                }
                return false;
            }
            env.cursor = env.limit - v_1;
            'lab1: loop {
                let v_2 = env.limit - env.cursor;
                'lab2: loop {
                    if !env.eq_s_b(&"in") {
                        break 'lab2;
                    }
                    if env.cursor > env.limit_backward {
                        break 'lab2;
                    }
                    if !env.slice_from("n") {
                        return false;
                    }
                    break 'lab1;
                }
                env.cursor = env.limit - v_2;
                if !env.slice_del() {
                    return false;
                }
                break 'lab1;
            }
        }
        2 => {
            let v_3 = env.limit - env.cursor;
            'lab3: loop {
                if !env.eq_s_b(&"h") {
                    break 'lab3;
                }
                if !r_R1(env, context) {
                    break 'lab3;
                }
                return false;
            }
            env.cursor = env.limit - v_3;
            let v_4 = env.limit - env.cursor;
            'lab4: loop {
                if !env.eq_s_b(&"en") {
                    break 'lab4;
                }
                if env.cursor > env.limit_backward {
                    break 'lab4;
                }
                return false;
            }
            env.cursor = env.limit - v_4;
            if !env.slice_del() {
                return false;
            }
        }
        _ => ()
    }
    return true
}

fn r_Lose_prefix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    if !env.eq_s(&"ge") {
        return false;
    }
    env.ket = env.cursor;
    let v_1 = env.cursor;
    if !env.hop(3) {
        return false;
    }
    env.cursor = v_1;
    let v_2 = env.cursor;
    'golab0: loop {
        let v_3 = env.cursor;
        'lab1: loop {
            'lab2: loop {
                let v_4 = env.cursor;
                'lab3: loop {
                    if !env.eq_s(&"ij") {
                        break 'lab3;
                    }
                    break 'lab2;
                }
                env.cursor = v_4;
                if !env.in_grouping(G_v, 97, 252) {
                    break 'lab1;
                }
                break 'lab2;
            }
            break 'golab0;
        }
        env.cursor = v_3;
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    'replab4: loop{
        let v_5 = env.cursor;
        'lab5: for _ in 0..1 {
            'lab6: loop {
                let v_6 = env.cursor;
                'lab7: loop {
                    if !env.eq_s(&"ij") {
                        break 'lab7;
                    }
                    break 'lab6;
                }
                env.cursor = v_6;
                if !env.in_grouping(G_v, 97, 252) {
                    break 'lab5;
                }
                break 'lab6;
            }
            continue 'replab4;
        }
        env.cursor = v_5;
        break 'replab4;
    }
    'lab8: loop {
        if env.cursor < env.limit {
            break 'lab8;
        }
        return false;
    }
    env.cursor = v_2;
    if (env.cursor + 2 >= env.limit || env.current.as_bytes()[(env.cursor + 2) as usize] as u8 >> 5 != 3 as u8 || ((1314818 as i32 >> (env.current.as_bytes()[(env.cursor + 2) as usize] as u8 & 0x1f)) & 1) == 0) {among_var = -1;}
    else {
        among_var = env.find_among(A_9, context);
    }
    match among_var {
        1 => {
            return false;
        }
        _ => ()
    }
    context.b_GE_removed = true;
    if !env.slice_del() {
        return false;
    }
    let v_7 = env.cursor;
    'lab9: loop {
        env.bra = env.cursor;
        if (env.cursor + 1 >= env.limit || (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 171 as u8 && env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 175 as u8)) {
            break 'lab9;
        }

        among_var = env.find_among(A_10, context);
        if among_var == 0 {
            break 'lab9;
        }
        env.ket = env.cursor;
        match among_var {
            1 => {
                if !env.slice_from("e") {
                    return false;
                }
            }
            2 => {
                if !env.slice_from("i") {
                    return false;
                }
            }
            _ => ()
        }
        break 'lab9;
    }
    env.cursor = v_7;
    return true
}

fn r_Lose_infix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor >= env.limit {
        return false;
    }
    env.next_char();
    'golab0: loop {
        'lab1: loop {
            env.bra = env.cursor;
            if !env.eq_s(&"ge") {
                break 'lab1;
            }
            env.ket = env.cursor;
            break 'golab0;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    let v_1 = env.cursor;
    if !env.hop(3) {
        return false;
    }
    env.cursor = v_1;
    let v_2 = env.cursor;
    'golab2: loop {
        let v_3 = env.cursor;
        'lab3: loop {
            'lab4: loop {
                let v_4 = env.cursor;
                'lab5: loop {
                    if !env.eq_s(&"ij") {
                        break 'lab5;
                    }
                    break 'lab4;
                }
                env.cursor = v_4;
                if !env.in_grouping(G_v, 97, 252) {
                    break 'lab3;
                }
                break 'lab4;
            }
            break 'golab2;
        }
        env.cursor = v_3;
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    'replab6: loop{
        let v_5 = env.cursor;
        'lab7: for _ in 0..1 {
            'lab8: loop {
                let v_6 = env.cursor;
                'lab9: loop {
                    if !env.eq_s(&"ij") {
                        break 'lab9;
                    }
                    break 'lab8;
                }
                env.cursor = v_6;
                if !env.in_grouping(G_v, 97, 252) {
                    break 'lab7;
                }
                break 'lab8;
            }
            continue 'replab6;
        }
        env.cursor = v_5;
        break 'replab6;
    }
    'lab10: loop {
        if env.cursor < env.limit {
            break 'lab10;
        }
        return false;
    }
    env.cursor = v_2;
    context.b_GE_removed = true;
    if !env.slice_del() {
        return false;
    }
    let v_7 = env.cursor;
    'lab11: loop {
        env.bra = env.cursor;
        if (env.cursor + 1 >= env.limit || (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 171 as u8 && env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 175 as u8)) {
            break 'lab11;
        }

        among_var = env.find_among(A_11, context);
        if among_var == 0 {
            break 'lab11;
        }
        env.ket = env.cursor;
        match among_var {
            1 => {
                if !env.slice_from("e") {
                    return false;
                }
            }
            2 => {
                if !env.slice_from("i") {
                    return false;
                }
            }
            _ => ()
        }
        break 'lab11;
    }
    env.cursor = v_7;
    return true
}

fn r_measure(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        'replab1: loop{
            'lab2: for _ in 0..1 {
                if !env.out_grouping(G_v, 97, 252) {
                    break 'lab2;
                }
                continue 'replab1;
            }
            break 'replab1;
        }
        let mut v_2 = 1;
        'replab3: loop{
            let v_3 = env.cursor;
            'lab4: for _ in 0..1 {
                'lab5: loop {
                    let v_4 = env.cursor;
                    'lab6: loop {
                        if !env.eq_s(&"ij") {
                            break 'lab6;
                        }
                        break 'lab5;
                    }
                    env.cursor = v_4;
                    if !env.in_grouping(G_v, 97, 252) {
                        break 'lab4;
                    }
                    break 'lab5;
                }
                v_2 -= 1;
                continue 'replab3;
            }
            env.cursor = v_3;
            break 'replab3;
        }
        if v_2 > 0 {
            break 'lab0;
        }
        if !env.out_grouping(G_v, 97, 252) {
            break 'lab0;
        }
        context.i_p1 = env.cursor;
        'replab7: loop{
            'lab8: for _ in 0..1 {
                if !env.out_grouping(G_v, 97, 252) {
                    break 'lab8;
                }
                continue 'replab7;
            }
            break 'replab7;
        }
        let mut v_5 = 1;
        'replab9: loop{
            let v_6 = env.cursor;
            'lab10: for _ in 0..1 {
                'lab11: loop {
                    let v_7 = env.cursor;
                    'lab12: loop {
                        if !env.eq_s(&"ij") {
                            break 'lab12;
                        }
                        break 'lab11;
                    }
                    env.cursor = v_7;
                    if !env.in_grouping(G_v, 97, 252) {
                        break 'lab10;
                    }
                    break 'lab11;
                }
                v_5 -= 1;
                continue 'replab9;
            }
            env.cursor = v_6;
            break 'replab9;
        }
        if v_5 > 0 {
            break 'lab0;
        }
        if !env.out_grouping(G_v, 97, 252) {
            break 'lab0;
        }
        context.i_p2 = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        b_GE_removed: false,
        b_stemmed: false,
        i_p2: 0,
        i_p1: 0,
        S_ch: String::new(),
    };
    context.b_stemmed = false;
    r_measure(env, context);
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        if !r_Step_1(env, context) {
            break 'lab0;
        }
        context.b_stemmed = true;
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab1: loop {
        if !r_Step_2(env, context) {
            break 'lab1;
        }
        context.b_stemmed = true;
        break 'lab1;
    }
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    'lab2: loop {
        if !r_Step_3(env, context) {
            break 'lab2;
        }
        context.b_stemmed = true;
        break 'lab2;
    }
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    'lab3: loop {
        if !r_Step_4(env, context) {
            break 'lab3;
        }
        context.b_stemmed = true;
        break 'lab3;
    }
    env.cursor = env.limit - v_4;
    env.cursor = env.limit_backward;
    context.b_GE_removed = false;
    let v_5 = env.cursor;
    'lab4: loop {
        let v_6 = env.cursor;
        if !r_Lose_prefix(env, context) {
            break 'lab4;
        }
        env.cursor = v_6;
        r_measure(env, context);
        break 'lab4;
    }
    env.cursor = v_5;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_7 = env.limit - env.cursor;
    'lab5: loop {
        if !context.b_GE_removed {
            break 'lab5;
        }
        context.b_stemmed = true;
        if !r_Step_1c(env, context) {
            break 'lab5;
        }
        break 'lab5;
    }
    env.cursor = env.limit - v_7;
    env.cursor = env.limit_backward;
    context.b_GE_removed = false;
    let v_8 = env.cursor;
    'lab6: loop {
        let v_9 = env.cursor;
        if !r_Lose_infix(env, context) {
            break 'lab6;
        }
        env.cursor = v_9;
        r_measure(env, context);
        break 'lab6;
    }
    env.cursor = v_8;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_10 = env.limit - env.cursor;
    'lab7: loop {
        if !context.b_GE_removed {
            break 'lab7;
        }
        context.b_stemmed = true;
        if !r_Step_1c(env, context) {
            break 'lab7;
        }
        break 'lab7;
    }
    env.cursor = env.limit - v_10;
    env.cursor = env.limit_backward;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_11 = env.limit - env.cursor;
    'lab8: loop {
        if !r_Step_7(env, context) {
            break 'lab8;
        }
        context.b_stemmed = true;
        break 'lab8;
    }
    env.cursor = env.limit - v_11;
    let v_12 = env.limit - env.cursor;
    'lab9: loop {
        if !context.b_stemmed {
            break 'lab9;
        }
        if !r_Step_6(env, context) {
            break 'lab9;
        }
        break 'lab9;
    }
    env.cursor = env.limit - v_12;
    env.cursor = env.limit_backward;
    return true
}
