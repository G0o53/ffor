#![no_std]

/*
 *   _____ _   _ _   _  ____ _____ ___ ___  _   _ ____  
 *  |  ___| | | | \ | |/ ___|_   _|_ _/ _ \| \ | / ___| 
 *  | |_  | | | |  \| | |     | |  | | | | |  \| \___ \ 
 *  |  _| | |_| | |\  | |___  | |  | | |_| | |\  |___) |
 *  |_|    \___/|_| \_|\____| |_| |___\___/|_| \_|____/ 
                                                    
*/

#[inline]
pub fn ffor8(cnt: i8, mut c: i8) -> i8 {
    if c < cnt { // checks if the current counter is less than what is wanted
        c += 1;  // if it is smaller, it adds one
        c
    } else {     // if not, it returns -1
        -1
    }
}

#[inline]
pub fn ffor16(cnt: i16, mut c: i16) -> i16 {
    if c < cnt {
        c += 1;
        c
    } else {
        -1
    }
}

#[inline]
pub fn ffor32(cnt: i32, mut c: i32) -> i32 {
    if c < cnt {
        c += 1;
        c
    } else {
        -1
    }
}

#[inline]
pub fn ffor64(cnt: i64, mut c: i64) -> i64 {
    if c < cnt {
        c += 1;
        c
    } else {
        -1
    }
}

#[inline]
pub fn ffor128(cnt: i128, mut c: i128) -> i128 {
    if c < cnt {
        c += 1;
        c
    } else {
        -1
    }
}
