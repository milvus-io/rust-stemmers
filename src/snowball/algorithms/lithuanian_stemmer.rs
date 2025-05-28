//! Generated from lithuanian.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 204] = &[
    Among("a", -1, -1, None),
    Among("ia", 0, -1, None),
    Among("eria", 1, -1, None),
    Among("osna", 0, -1, None),
    Among("iosna", 3, -1, None),
    Among("uosna", 3, -1, None),
    Among("iuosna", 5, -1, None),
    Among("ysna", 0, -1, None),
    Among("ėsna", 0, -1, None),
    Among("e", -1, -1, None),
    Among("ie", 9, -1, None),
    Among("enie", 10, -1, None),
    Among("erie", 10, -1, None),
    Among("oje", 9, -1, None),
    Among("ioje", 13, -1, None),
    Among("uje", 9, -1, None),
    Among("iuje", 15, -1, None),
    Among("yje", 9, -1, None),
    Among("enyje", 17, -1, None),
    Among("eryje", 17, -1, None),
    Among("ėje", 9, -1, None),
    Among("ame", 9, -1, None),
    Among("iame", 21, -1, None),
    Among("sime", 9, -1, None),
    Among("ome", 9, -1, None),
    Among("ėme", 9, -1, None),
    Among("tumėme", 25, -1, None),
    Among("ose", 9, -1, None),
    Among("iose", 27, -1, None),
    Among("uose", 27, -1, None),
    Among("iuose", 29, -1, None),
    Among("yse", 9, -1, None),
    Among("enyse", 31, -1, None),
    Among("eryse", 31, -1, None),
    Among("ėse", 9, -1, None),
    Among("ate", 9, -1, None),
    Among("iate", 35, -1, None),
    Among("ite", 9, -1, None),
    Among("kite", 37, -1, None),
    Among("site", 37, -1, None),
    Among("ote", 9, -1, None),
    Among("tute", 9, -1, None),
    Among("ėte", 9, -1, None),
    Among("tumėte", 42, -1, None),
    Among("i", -1, -1, None),
    Among("ai", 44, -1, None),
    Among("iai", 45, -1, None),
    Among("eriai", 46, -1, None),
    Among("ei", 44, -1, None),
    Among("tumei", 48, -1, None),
    Among("ki", 44, -1, None),
    Among("imi", 44, -1, None),
    Among("erimi", 51, -1, None),
    Among("umi", 44, -1, None),
    Among("iumi", 53, -1, None),
    Among("si", 44, -1, None),
    Among("asi", 55, -1, None),
    Among("iasi", 56, -1, None),
    Among("esi", 55, -1, None),
    Among("iesi", 58, -1, None),
    Among("siesi", 59, -1, None),
    Among("isi", 55, -1, None),
    Among("aisi", 61, -1, None),
    Among("eisi", 61, -1, None),
    Among("tumeisi", 63, -1, None),
    Among("uisi", 61, -1, None),
    Among("osi", 55, -1, None),
    Among("ėjosi", 66, -1, None),
    Among("uosi", 66, -1, None),
    Among("iuosi", 68, -1, None),
    Among("siuosi", 69, -1, None),
    Among("usi", 55, -1, None),
    Among("ausi", 71, -1, None),
    Among("čiausi", 72, -1, None),
    Among("ąsi", 55, -1, None),
    Among("ėsi", 55, -1, None),
    Among("ųsi", 55, -1, None),
    Among("tųsi", 76, -1, None),
    Among("ti", 44, -1, None),
    Among("enti", 78, -1, None),
    Among("inti", 78, -1, None),
    Among("oti", 78, -1, None),
    Among("ioti", 81, -1, None),
    Among("uoti", 81, -1, None),
    Among("iuoti", 83, -1, None),
    Among("auti", 78, -1, None),
    Among("iauti", 85, -1, None),
    Among("yti", 78, -1, None),
    Among("ėti", 78, -1, None),
    Among("telėti", 88, -1, None),
    Among("inėti", 88, -1, None),
    Among("terėti", 88, -1, None),
    Among("ui", 44, -1, None),
    Among("iui", 92, -1, None),
    Among("eniui", 93, -1, None),
    Among("oj", -1, -1, None),
    Among("ėj", -1, -1, None),
    Among("k", -1, -1, None),
    Among("am", -1, -1, None),
    Among("iam", 98, -1, None),
    Among("iem", -1, -1, None),
    Among("im", -1, -1, None),
    Among("sim", 101, -1, None),
    Among("om", -1, -1, None),
    Among("tum", -1, -1, None),
    Among("ėm", -1, -1, None),
    Among("tumėm", 105, -1, None),
    Among("an", -1, -1, None),
    Among("on", -1, -1, None),
    Among("ion", 108, -1, None),
    Among("un", -1, -1, None),
    Among("iun", 110, -1, None),
    Among("ėn", -1, -1, None),
    Among("o", -1, -1, None),
    Among("io", 113, -1, None),
    Among("enio", 114, -1, None),
    Among("ėjo", 113, -1, None),
    Among("uo", 113, -1, None),
    Among("s", -1, -1, None),
    Among("as", 118, -1, None),
    Among("ias", 119, -1, None),
    Among("es", 118, -1, None),
    Among("ies", 121, -1, None),
    Among("is", 118, -1, None),
    Among("ais", 123, -1, None),
    Among("iais", 124, -1, None),
    Among("tumeis", 123, -1, None),
    Among("imis", 123, -1, None),
    Among("enimis", 127, -1, None),
    Among("omis", 123, -1, None),
    Among("iomis", 129, -1, None),
    Among("umis", 123, -1, None),
    Among("ėmis", 123, -1, None),
    Among("enis", 123, -1, None),
    Among("asis", 123, -1, None),
    Among("ysis", 123, -1, None),
    Among("ams", 118, -1, None),
    Among("iams", 136, -1, None),
    Among("iems", 118, -1, None),
    Among("ims", 118, -1, None),
    Among("enims", 139, -1, None),
    Among("erims", 139, -1, None),
    Among("oms", 118, -1, None),
    Among("ioms", 142, -1, None),
    Among("ums", 118, -1, None),
    Among("ėms", 118, -1, None),
    Among("ens", 118, -1, None),
    Among("os", 118, -1, None),
    Among("ios", 147, -1, None),
    Among("uos", 147, -1, None),
    Among("iuos", 149, -1, None),
    Among("ers", 118, -1, None),
    Among("us", 118, -1, None),
    Among("aus", 152, -1, None),
    Among("iaus", 153, -1, None),
    Among("ius", 152, -1, None),
    Among("ys", 118, -1, None),
    Among("enys", 156, -1, None),
    Among("erys", 156, -1, None),
    Among("ąs", 118, -1, None),
    Among("iąs", 159, -1, None),
    Among("ės", 118, -1, None),
    Among("amės", 161, -1, None),
    Among("iamės", 162, -1, None),
    Among("imės", 161, -1, None),
    Among("kimės", 164, -1, None),
    Among("simės", 164, -1, None),
    Among("omės", 161, -1, None),
    Among("ėmės", 161, -1, None),
    Among("tumėmės", 168, -1, None),
    Among("atės", 161, -1, None),
    Among("iatės", 170, -1, None),
    Among("sitės", 161, -1, None),
    Among("otės", 161, -1, None),
    Among("ėtės", 161, -1, None),
    Among("tumėtės", 174, -1, None),
    Among("ūs", 118, -1, None),
    Among("įs", 118, -1, None),
    Among("tųs", 118, -1, None),
    Among("at", -1, -1, None),
    Among("iat", 179, -1, None),
    Among("it", -1, -1, None),
    Among("sit", 181, -1, None),
    Among("ot", -1, -1, None),
    Among("ėt", -1, -1, None),
    Among("tumėt", 184, -1, None),
    Among("u", -1, -1, None),
    Among("au", 186, -1, None),
    Among("iau", 187, -1, None),
    Among("čiau", 188, -1, None),
    Among("iu", 186, -1, None),
    Among("eniu", 190, -1, None),
    Among("siu", 190, -1, None),
    Among("y", -1, -1, None),
    Among("ą", -1, -1, None),
    Among("ią", 194, -1, None),
    Among("ė", -1, -1, None),
    Among("ę", -1, -1, None),
    Among("į", -1, -1, None),
    Among("enį", 198, -1, None),
    Among("erį", 198, -1, None),
    Among("ų", -1, -1, None),
    Among("ių", 201, -1, None),
    Among("erų", 201, -1, None),
];

static A_1: &'static [Among<Context>; 62] = &[
    Among("ing", -1, -1, None),
    Among("aj", -1, -1, None),
    Among("iaj", 1, -1, None),
    Among("iej", -1, -1, None),
    Among("oj", -1, -1, None),
    Among("ioj", 4, -1, None),
    Among("uoj", 4, -1, None),
    Among("iuoj", 6, -1, None),
    Among("auj", -1, -1, None),
    Among("ąj", -1, -1, None),
    Among("iąj", 9, -1, None),
    Among("ėj", -1, -1, None),
    Among("ųj", -1, -1, None),
    Among("iųj", 12, -1, None),
    Among("ok", -1, -1, None),
    Among("iok", 14, -1, None),
    Among("iuk", -1, -1, None),
    Among("uliuk", 16, -1, None),
    Among("učiuk", 16, -1, None),
    Among("išk", -1, -1, None),
    Among("iul", -1, -1, None),
    Among("yl", -1, -1, None),
    Among("ėl", -1, -1, None),
    Among("am", -1, -1, None),
    Among("dam", 23, -1, None),
    Among("jam", 23, -1, None),
    Among("zgan", -1, -1, None),
    Among("ain", -1, -1, None),
    Among("esn", -1, -1, None),
    Among("op", -1, -1, None),
    Among("iop", 29, -1, None),
    Among("ias", -1, -1, None),
    Among("ies", -1, -1, None),
    Among("ais", -1, -1, None),
    Among("iais", 33, -1, None),
    Among("os", -1, -1, None),
    Among("ios", 35, -1, None),
    Among("uos", 35, -1, None),
    Among("iuos", 37, -1, None),
    Among("aus", -1, -1, None),
    Among("iaus", 39, -1, None),
    Among("ąs", -1, -1, None),
    Among("iąs", 41, -1, None),
    Among("ęs", -1, -1, None),
    Among("utėait", -1, -1, None),
    Among("ant", -1, -1, None),
    Among("iant", 45, -1, None),
    Among("siant", 46, -1, None),
    Among("int", -1, -1, None),
    Among("ot", -1, -1, None),
    Among("uot", 49, -1, None),
    Among("iuot", 50, -1, None),
    Among("yt", -1, -1, None),
    Among("ėt", -1, -1, None),
    Among("ykšt", -1, -1, None),
    Among("iau", -1, -1, None),
    Among("dav", -1, -1, None),
    Among("sv", -1, -1, None),
    Among("šv", -1, -1, None),
    Among("ykšč", -1, -1, None),
    Among("ę", -1, -1, None),
    Among("ėję", 60, -1, None),
];

static A_2: &'static [Among<Context>; 11] = &[
    Among("ojime", -1, 7, None),
    Among("ėjime", -1, 3, None),
    Among("avime", -1, 6, None),
    Among("okate", -1, 8, None),
    Among("aite", -1, 1, None),
    Among("uote", -1, 2, None),
    Among("asius", -1, 5, None),
    Among("okatės", -1, 8, None),
    Among("aitės", -1, 1, None),
    Among("uotės", -1, 2, None),
    Among("esiu", -1, 4, None),
];

static A_3: &'static [Among<Context>; 2] = &[
    Among("č", -1, 1, None),
    Among("dž", -1, 2, None),
];

static A_4: &'static [Among<Context>; 1] = &[
    Among("gd", -1, 1, None),
];

static G_v: &'static [u8; 35] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 64, 1, 0, 64, 0, 0, 0, 0, 0, 0, 0, 4, 4];

#[derive(Clone)]
struct Context {
    i_p1: i32,
}

fn r_step1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if env.find_among_b(A_0, context) == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    if !env.slice_del() {
        return false;
    }
    return true
}

fn r_step2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'replab0: loop{
        let v_1 = env.limit - env.cursor;
        'lab1: for _ in 0..1 {
            if env.cursor < context.i_p1 {
                break 'lab1;
            }
            let v_2 = env.limit_backward;
            env.limit_backward = context.i_p1;
            env.ket = env.cursor;
            if env.find_among_b(A_1, context) == 0 {
                env.limit_backward = v_2;
                break 'lab1;
            }
            env.bra = env.cursor;
            env.limit_backward = v_2;
            if !env.slice_del() {
                return false;
            }
            continue 'replab0;
        }
        env.cursor = env.limit - v_1;
        break 'replab0;
    }
    return true
}

fn r_fix_conflicts(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 3 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((2621472 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !env.slice_from("aitė") {
                return false;
            }
        }
        2 => {
            if !env.slice_from("uotė") {
                return false;
            }
        }
        3 => {
            if !env.slice_from("ėjimas") {
                return false;
            }
        }
        4 => {
            if !env.slice_from("esys") {
                return false;
            }
        }
        5 => {
            if !env.slice_from("asys") {
                return false;
            }
        }
        6 => {
            if !env.slice_from("avimas") {
                return false;
            }
        }
        7 => {
            if !env.slice_from("ojimas") {
                return false;
            }
        }
        8 => {
            if !env.slice_from("okatė") {
                return false;
            }
        }
        _ => ()
    }
    return true
}

fn r_fix_chdz(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 141 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 190 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_3, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !env.slice_from("t") {
                return false;
            }
        }
        2 => {
            if !env.slice_from("d") {
                return false;
            }
        }
        _ => ()
    }
    return true
}

fn r_fix_gd(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 100 as u8) {
        return false;
    }

    if env.find_among_b(A_4, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !env.slice_from("g") {
        return false;
    }
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p1: 0,
    };
    context.i_p1 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        let v_2 = env.cursor;
        'lab1: loop {
            let v_3 = env.cursor;
            if !env.eq_s(&"a") {
                env.cursor = v_2;
                break 'lab1;
            }
            env.cursor = v_3;
            if (env.current.chars().count() as i32) <= 6{
                env.cursor = v_2;
                break 'lab1;
            }
            if env.cursor >= env.limit {
                env.cursor = v_2;
                break 'lab1;
            }
            env.next_char();
            break 'lab1;
        }
        if !env.go_out_grouping(G_v, 97, 371) {
            break 'lab0;
        }
env.next_char();        if !env.go_in_grouping(G_v, 97, 371) {
            break 'lab0;
        }
env.next_char();        context.i_p1 = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_4 = env.limit - env.cursor;
    r_fix_conflicts(env, context);
    env.cursor = env.limit - v_4;
    let v_5 = env.limit - env.cursor;
    r_step1(env, context);
    env.cursor = env.limit - v_5;
    let v_6 = env.limit - env.cursor;
    r_fix_chdz(env, context);
    env.cursor = env.limit - v_6;
    let v_7 = env.limit - env.cursor;
    r_step2(env, context);
    env.cursor = env.limit - v_7;
    let v_8 = env.limit - env.cursor;
    r_fix_chdz(env, context);
    env.cursor = env.limit - v_8;
    let v_9 = env.limit - env.cursor;
    r_fix_gd(env, context);
    env.cursor = env.limit - v_9;
    env.cursor = env.limit_backward;
    return true
}
