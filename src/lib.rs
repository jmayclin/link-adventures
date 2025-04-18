extern "C" {
    static stuff: *const u8;

    fn get_stuff() -> *const u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    // SIGSEGV
    // #[test]
    // fn test_reads() {
    //     unsafe {
    //         println!("{:p}", stuff);
    //         assert_eq!(*stuff, 3);
    //     }
    // }

    #[test]
    fn similarity() {
        unsafe {
            assert_eq!(stuff, get_stuff());
        }
    }

    #[test]
    fn getter() {
        unsafe {
            assert_eq!(*get_stuff(), 3);
        }
    }
}
